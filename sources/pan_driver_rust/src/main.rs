use config::{Config, ConfigError};
use midir::{MidiOutput, MidiOutputPort};
use serial_to_midi_lib::*;
use serialport::{SerialPort, SerialPortInfo, SerialPortType};
use sysinfo::Pid;
use std::{
    error::Error,
    io::{BufRead, BufReader, ErrorKind, Write, self},
    process::{Command, Child},
    thread,
    time::Duration, 
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut system = sysinfo::System::new();
    
    let Settings {
        baud_rate, 
        loopmidi_path, 
        virtual_midi_port_name} = Settings::load_from_file()?;

    loop {
    print!("Starting loopMIDI..");
    let mut loopmidi_process = LoopMidiProcess::start_or_find(&loopmidi_path, &mut system);

    print!("Opening serial port..");
    let mut reader = get_pan_reader(baud_rate);

    print!("Opening virtual MIDI port..");
    let midi_out = match MidiOutput::new("The Pan Output") {
        Ok(out) => out,
        Err(msg) => {eprintln!("Error creating MIDI output. {msg}"); continue},
    };
    let midi_port: MidiOutputPort = match find_virtual_midi_port_by_name(&virtual_midi_port_name, &midi_out) {
        Some(port) => port,
        None => {eprintln!("Couldn't find virtual midi port {virtual_midi_port_name}") ; continue},
    };
    
    let mut midi_connection = match midi_out.connect(&midi_port, &virtual_midi_port_name) {
        Ok(connection) => connection,
        Err(msg) => {
            eprintln!("Couldn't connect to the virtual midi port. {msg}"); continue; 
        }
    };

    println!("Connected to The Pan.");
    let mut pan = Pan::new();
    let mut timeout_counter = 0;
    let serial_ports = match serialport::available_ports() {
        Ok(ports) => ports,
        Err(_) => continue, // If no ports are available, restart
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
                if let Err(err) = midi_connection.send(msg.to_midi().as_slice()) {
                    eprintln!("Failed to send MIDI message: {err} Restarting.");
                    break;
                }
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
    } // main loop
}

/// Opens the Pan port and aquires a BufReader to it. Blocks execution.
fn get_pan_reader(baud_rate: u32) -> BufReader<Box<dyn SerialPort>>{
    loop {
        print!(".");
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(500));

        match open_pan_port(baud_rate) { 
            Ok(reader) => break reader,
            Err(_) => {
                eprintln!("Couldn't find The Pan.");
            }
        };
    }
}
// When the device is not plugged in this causes the program to freeze.
fn open_pan_port(baud_rate: u32) -> std::io::Result<BufReader<Box<dyn SerialPort>>> {
    loop {
        let ports = serialport::available_ports()?;
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
impl LoopMidiProcess {
    fn start_or_find(loopmidi_path: &String, system: &mut sysinfo::System) -> Self {
        loop {
            print!(".");
            let _ = io::stdout().flush();

            system.refresh_processes();
            if let Some(process) = system.processes_by_name("loopMIDI").next() {
                println!("loopMIDI is already running.");
                break LoopMidiProcess::External(process.pid()) // loopMIDI is running, but is not started by this program
            };
            match Command::new(loopmidi_path).spawn() {
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
        }
    }
}

struct Settings {
    baud_rate: u32,
    loopmidi_path: String,
    virtual_midi_port_name: String
}

impl Settings {
    fn load_from_file() -> Result<Self, ConfigError>  {
        let settings = Config::builder()
            .add_source(config::File::with_name("config"))
            .build()?;

        let baud_rate = settings
            .get_int("port.serial_port_baudrate")? as u32;

        let loopmidi_path = settings
            .get_string("loopmidi.path")?;

        let virtual_midi_port_name = settings
            .get_string("port.virtual_midi_port_name")?;
        
        Ok(Settings {
            baud_rate,
            loopmidi_path,
            virtual_midi_port_name
        })
    }
}

fn find_virtual_midi_port_by_name(name: &String, midi_out: &MidiOutput) -> Option<MidiOutputPort> {
    let mut retry_count = 0;
    loop {
        print!(".");
        let _ = io::stdout().flush();
        // *port_list_buf = midi_out.ports();
        match midi_out.ports()
            .iter()
            .find(|port| midi_out.port_name(port).unwrap_or_default() == *name)
        {
            Some(port) => {return Some(port.to_owned())},
            None => {
                eprintln!("Virtual MIDI port not found. Has loopMIDI started?");
                thread::sleep(Duration::from_millis(400));
                retry_count += 1;
                if retry_count > 25 {
                    eprintln!("Virtual MIDI port could not be opened. Restarting.");
                    return None;
                }
                continue;
            }
        };
    }
}

