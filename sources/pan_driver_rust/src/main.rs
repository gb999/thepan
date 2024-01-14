use config::Config;
use midir::{MidiOutput, MidiOutputPort};
use serial_to_midi_lib::*;
use serialport::{SerialPort, SerialPortInfo, SerialPortType};
use sysinfo::Pid;
use std::{
    error::Error,
    io::{BufRead, BufReader, ErrorKind, Write, self},
    process::{Command, Child},
    thread,
    time::Duration
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut system = sysinfo::System::new();
    // Read config file
    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;

    let baudrate = settings
        .get_int("port.serial_port_baudrate")
        .expect("Baudrate in config is not a number.") as u32;

    // TODO: It would be nice to create a virtual midi port and remove this dependency.
    // But currently it's not possible on Windows, as far as I know.
    let loopmidi_path = settings
        .get_string("loopmidi.path")
        .expect("No path provided to loopMIDI");

    let virtual_midi_port_name = settings
        .get_string("port.virtual_midi_port_name")
        .expect("No name provided for the virtual port.");

    'main: loop {
    print!("Starting loopMIDI..");
    let mut loopmidi_process = loop {
        print!(".");
        system.refresh_processes();
        if let Some(process) = system.processes_by_name("loopMIDI").next() {
            println!("loopMIDI is already running.");
            break LoopMidiProcess::External(process.pid()) // loopMIDI is running, but is not started by this program
        };
        match Command::new(&loopmidi_path).spawn() {
            Ok(process) => {
                println!("loopMIDI started.");
                break LoopMidiProcess::Owned(process);
            }
            Err(msg) => {
                thread::sleep(Duration::from_millis(400));
                eprintln!("Failed to start loopMIDI. {msg}");
                continue;
            }
        }
    };
    print!("Opening serial port..");
    let mut reader = loop {
        print!(".");
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(500));

        match open_pan_port(baudrate) { 
            Ok(reader) => break reader,
            Err(_) => {
                eprintln!("Couldn't find The Pan.");
            }
        };
    };
    print!("Opening virtual MIDI port..");
    let midi_out = MidiOutput::new("The Pan Output")?;
    let mut midi_ports;
    let mut retry_count = 0;
    let midi_port: &MidiOutputPort = loop {
        print!(".");
        midi_ports = midi_out.ports();
        match &midi_ports
            .iter()
            .find(|port| midi_out.port_name(port).unwrap() == virtual_midi_port_name)
        {
            Some(port) => break port,
            None => {
                eprintln!("Virtual MIDI port not found. Has loopMIDI started?");
                thread::sleep(Duration::from_millis(400));
                retry_count += 1;
                if retry_count > 25 {
                    eprintln!("Virtual MIDI port could not be opened. Restarting.");
                    continue 'main;
                }
                continue;
            }
        }
    };
    
    let mut midi_connection = match midi_out
        .connect(&midi_port, &virtual_midi_port_name) {
            Ok(connection) => connection,
            Err(msg) => {
                eprintln!("Couldn't connect to the virtual midi port. {msg}");
                continue; // restart the main loop
            }
        };
    println!("Connected to The Pan.");
    let mut pan = Pan::new();
    let mut timeout_counter = 0;
    let serial_ports = match serialport::available_ports() {
        Ok(ports) => ports,
        Err(_) => break, // If no ports are available, restart
    };
    let mut line = String::new();

    loop {
        match loopmidi_process {
            // If process is owned by this program, check if it exited
            LoopMidiProcess::Owned(ref mut process) => {
                match process.try_wait() {
                    Ok(exit_status) => {
                        if let Some(_) = exit_status {
                            break;
                        };
                    }
                    Err(e) => {
                        eprintln!("Error attempting to wait loopMIDI: {e}");
                        break;
                    }
                };
            }
            LoopMidiProcess::External(pid) => { // Check if loopMIDI is still running
               system.refresh_processes();
               if let None = system.process(pid) {
                eprintln!("loopMIDI is no longer running");
                break;
               }
            }
        };
        
        line.clear();
        match reader.read_line(&mut line) {
            Ok(_byte_count) => {
                let command = parse_line(&line);
                let msg = pan.handle_command(command);
                midi_connection.send(msg.to_midi().as_slice()).unwrap();
            }
            Err(ref e) if e.kind() == ErrorKind::TimedOut => {
                // Timeout occurred, continue reading
                timeout_counter += 1; 
                if timeout_counter < 1000 {
                    continue;
                }
                // If 1000 timeouts occurred check if there was a change in usb ports
                match serialport::available_ports() {
                    Ok(ports) => {
                        if ports == serial_ports {
                            timeout_counter = 0;
                        } else {
                            eprintln!("Serial ports changed. Restarting.");
                            break; // If there was a change it's easier to restart
                        }
                    },
                    Err(_) => break, // If there are no ports available, restart
                }

            }
            Err(msg) => {
                eprintln!("Error reading port: {}", msg);
                break;
            }
        }
    }
    }

    Ok(())
}

// When the device is not plugged in this causes the program to freeze.
fn open_pan_port(baud_rate: u32) -> std::io::Result<BufReader<Box<dyn SerialPort>>> {
    let ports = serialport::available_ports()?;
    loop {
        for SerialPortInfo {
            port_name,
            port_type,
        } in &ports
        {
            if let SerialPortType::UsbPort(_) = port_type {
                let mut port = serialport::new(port_name, baud_rate)
                    .timeout(Duration::from_millis(10))
                    .open()?;
                port.write_all(b"Who are you?\n")?;
                let mut reader = BufReader::new(port);
                let mut buf = String::new();
                reader.read_line(&mut buf)?; // b"I am The Pan\r\n"
                if buf.trim() == "I am The Pan" {
                    return Ok(reader);
                }
            }
        }
    }
}

enum LoopMidiProcess {
    Owned(Child),
    External(Pid),
}