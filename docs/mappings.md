# Mapping Encoder Gestures to plugins
The [python script](../sources/fl_script) shows a few examples, how a mapping of gestures to plugin parameters can be implemented. The gestures, when not handled by a mapping can be linked to parameters in FL Studio and they can increment or decrement values of parameters.
Also see [FL Studio MIDI Scripting for more info.](https://www.image-line.com/fl-studio-learning/fl-studio-online-manual/html/midi_scripting.htm )

Here are two implemented mappings for **Fruity Parametric EQ 2** and **Fruity Delay 3** and a suggested mapping for **Fruity Reeverb 2**.

# Fruity Parametric EQ 2 Control Mappings
| Encoder Idx | 1-5 |
| ------------| --- |
| Rotate      | Level|
| Hold and rotate | Band Width |
| Double press then rotate | Band Type |
| Double press and rotate | Band Order |

| Pot Idx | 1-5       |
| ------- | --------- |
| Rotate  | Frequency |


# Fruity Delay 3 Control Mappings
| Encoder Idx               | 1             | 2               | 3                   | 4                       | 5                |
| ------------------------- | ------------- | --------------- | ------------------- | ----------------------- | ---------------- |
| Button Press              | Tempo sync    | -               | Keep Pitch          | -                       | Limit/Sat        |
| Rotate                    | Time (step)   | Stereo Offset   | Feedback Level      | SMP Rate                | Diffusion LVL    |
| Hold and rotate           | Time (smooth) | -               | Feedback Cutoff     | Bits                    | Diffusion Spread |
| Double press then rotate  | Out Wet Level | Delay Model     |Feedback Filter mode |   -                     |-                 |
| Double press, hold rotate | -             |   -             | -                   |     -                   | -                |
| Alt + rotate              | Mod Time      | Smoothing       |              -      | Distortion level        |-                 |
| Alt + hold + rotate       | Mod Rate      | -               | Feedback Cutoff Mod | Distortion knee         |Feedback Resonance|


# Fruity Reeverb 2 Mappings
| Encoder Idx | 1 | 2 | 3 | 4 | 5 |
|-|-|-|-|-|-|
| Rotate                   | Decay     | Size | Diffusion | Wet Level | High Cut |
| Hold and Rotate          |   -       | Delay| -         |  -        | Low Cut  |         
| Double press then rotate | -         | -    | -         | -         | -        |
| Alt + rotate             | Mod Time  | -    | -         | -         | -        |  
| Alt + hold + rotate      | Mod speed | -    | -         | -         | -        |

