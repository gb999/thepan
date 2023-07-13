# name=gb99 The Pan 
# url=https://gb999.github.io/thepan/

# This import section is loading the back-end code required to execute the script. You may not need all modules that are available for all scripts.

import ui
import midi
import device
import general
import mixer
import plugins

import math
import time

import utils
import channels

from midi import *

POT_BASE_IDX = 0x66
ENCODER_BASE_IDX = 0x0E



# Plugin Names
PARAM_EQ_2 = "Fruity parametric EQ 2"
REEVERB_2 = "Fruity Reeverb 2"
DELAY_3 = "Fruity Delay 3"


# Parametric EQ 2 Parameters
EQ_PARAM_MAINLEVEL_INDEX = 35
EQ_PARAM_LEVEL_OFFSET = 0
EQ_PARAM_FREQ_OFFSET = 7
EQ_PARAM_BANDWIDTH_OFFSET = 14
EQ_PARAM_BANDTYPE_OFFSET = 21
EQ_PARAM_BANDORDER_OFFSET = 28
EQ_USED_PARAMS = [0, 1, 3, 5, 6]

# Encoder Gestures:
# 0:srel  1: drel  2: singlepress  3: dpress
EQ_ENCODER_GESTURE_TO_PARAMETER_OFFSET = (
	EQ_PARAM_LEVEL_OFFSET, 		# Rotate
	EQ_PARAM_BANDTYPE_OFFSET, 	# Double Press then rotate
	EQ_PARAM_BANDWIDTH_OFFSET, 	# Hold and rotate
	EQ_PARAM_BANDORDER_OFFSET	# Double press, hold and rotate
)


# Delay 3 Parameters
DELAY_PARAM_MODEL_INDEX = 1
DELAY_PARAM_TEMPOSYNC_INDEX = 2  
DELAY_PARAM_KEEPPITCH_INDEX = 3
DELAY_PARAM_TIME_INDEX = 4
DELAY_PARAM_STEREOOFFSET_INDEX = 5
DELAY_PARAM_SMOOTHING_INDEX = 6 
DELAY_PARAM_STEREOOFFSET_INDEX = 7
DELAY_PARAM_MODRATE_INDEX = 8
DELAY_PARAM_MODTIME_INDEX = 9
DELAY_PARAM_MODCUTOFF_INDEX = 10
DELAY_PARAM_DIFFUSIONLVL_INDEX = 11
DELAY_PARAM_DIFFUSIONSPREAD_INDEX = 12
DELAY_PARAM_DISTORTIONMODE_INDEX = 13
DELAY_PARAM_FEEDBACKLVL_INDEX = 14
DELAY_PARAM_FILTERMODE_INDEX = 15
DELAY_PARAM_FILTERCUTOFF_INDEX = 16
DELAY_PARAM_FILTERRES_INDEX = 17
DELAY_PARAM_DISTORTIONLVL_INDEX = 18
DELAY_PARAM_DISTORTIONKNEE_INDEX = 19
DELAY_PARAM_SMPRATE_INDEX = 21
DELAY_PARAM_BITS_INDEX = 22
DELAY_PARAM_OUTPUTWET_INDEX = 23


class Parameter():
	STEP_MODE_MIN_TIME = 60
	def __DefaultCalculateStep(self, value):
		currentTime = round(time.time() * 1000)
		res = 0
		if self.stepCount <= 1: 
			res = value / 64
		elif currentTime - self.lastStepTime >= self.STEP_MODE_MIN_TIME:
			res = math.copysign(1 / (self.stepCount -1), value)
		self.lastStepTime = currentTime
		return res
	def __init__(self, idx = None, stepCount = 0, calcStepFn = __DefaultCalculateStep):
		self.idx = idx 
		self.stepCount = stepCount  
		self.calcStepFn = calcStepFn
		self.lastStepTime = 0

	def CalculateStep(self, value):
		return self.calcStepFn(self, value)
	
	def Switch(self, x):
		return (x+1) % 2
		

class GestureToParamMapping(dict):
	def __init__(self,
	rotate 				= (0,), 
	holdrotate 			= (0,),
	doublethenrotate  	= (0,),
	doubleholdrotate  	= (0,),
	altrotate  			= (0,),
	altholdrotate  		= (0,),
	buttondown 			= (0,),
	buttonup  			= (0,),
	):
		super().__init__(self,
			rotate=				Parameter(*rotate),
			holdrotate=			Parameter(*holdrotate),
			doublethenrotate=	Parameter(*doublethenrotate),
			doubleholdrotate=	Parameter(*doubleholdrotate),
			altrotate=			Parameter(*altrotate),
			altholdrotate=		Parameter(*altholdrotate),
			buttondown=			Parameter(*buttondown),
			buttonup=			Parameter(*buttonup)
		)

class EncoderMapping(tuple):
	def __new__(cls, e1, e2, e3, e4, e5):
		return super(EncoderMapping, cls).__new__(cls, tuple((e1,e2,e3,e4,e5)))


# Put comma after index, to create a tuple

DELAY_3_E1_GESTUREMAPPING = GestureToParamMapping(
	rotate=				(DELAY_PARAM_TIME_INDEX, 16), 	#time step
	holdrotate=			(DELAY_PARAM_TIME_INDEX, 0), #time smooth
	doublethenrotate=	(DELAY_PARAM_OUTPUTWET_INDEX,),
	altrotate=			(DELAY_PARAM_MODTIME_INDEX,),
	altholdrotate=		(DELAY_PARAM_MODRATE_INDEX,),
	buttonup=			(DELAY_PARAM_TEMPOSYNC_INDEX, 0, Parameter.Switch)

)
DELAY_3_E2_GESTUREMAPPING = GestureToParamMapping(
	rotate=				(DELAY_PARAM_STEREOOFFSET_INDEX,), 	
	doublethenrotate=	(DELAY_PARAM_MODEL_INDEX, 4),
	altrotate=			(DELAY_PARAM_SMOOTHING_INDEX,),

)
DELAY_3_E3_GESTUREMAPPING = GestureToParamMapping(
	rotate=				(DELAY_PARAM_FEEDBACKLVL_INDEX,), 	
	holdrotate=			(DELAY_PARAM_FILTERCUTOFF_INDEX,), 
	doublethenrotate=	(DELAY_PARAM_FILTERMODE_INDEX, 4),
	altholdrotate=		(DELAY_PARAM_MODCUTOFF_INDEX,),
	buttonup=			(DELAY_PARAM_KEEPPITCH_INDEX,0,Parameter.Switch)

)
DELAY_3_E4_GESTUREMAPPING = GestureToParamMapping(
	rotate=				(DELAY_PARAM_SMPRATE_INDEX,), 	
	holdrotate=			(DELAY_PARAM_BITS_INDEX,), 
	altrotate=			(DELAY_PARAM_DISTORTIONLVL_INDEX,),
	altholdrotate=		(DELAY_PARAM_DISTORTIONKNEE_INDEX,)

)
DELAY_3_E5_GESTUREMAPPING = GestureToParamMapping(
	rotate=				(DELAY_PARAM_DIFFUSIONLVL_INDEX,), 	
	holdrotate=			(DELAY_PARAM_DIFFUSIONSPREAD_INDEX,), 
	altholdrotate=		(DELAY_PARAM_FILTERRES_INDEX,),
	buttonup=			(DELAY_PARAM_DISTORTIONMODE_INDEX, 0, Parameter.Switch)
)

DELAY_3_MAPPING = EncoderMapping(
	DELAY_3_E1_GESTUREMAPPING,
	DELAY_3_E2_GESTUREMAPPING,
	DELAY_3_E3_GESTUREMAPPING,
	DELAY_3_E4_GESTUREMAPPING,
	DELAY_3_E5_GESTUREMAPPING,
)


EQ_PARAM_MAINLEVEL_INDEX = 35
EQ_PARAM_LEVEL_OFFSET = 0
EQ_PARAM_FREQ_OFFSET = 7
EQ_PARAM_BANDWIDTH_OFFSET = 14
EQ_PARAM_BANDTYPE_OFFSET = 21
EQ_PARAM_BANDORDER_OFFSET = 28
EQ_USED_PARAMS = [0, 1, 3, 5, 6]


PARAM_EQ_2_MAPPING = ()


for i in range(0,5):
	param_idx_offset = EQ_USED_PARAMS[i]
	encoder_i_gesturemapping = GestureToParamMapping(
		rotate=				(EQ_PARAM_LEVEL_OFFSET + param_idx_offset,), 	
		holdrotate=			(EQ_PARAM_BANDWIDTH_OFFSET + param_idx_offset,), 
		doublethenrotate=	(EQ_PARAM_BANDTYPE_OFFSET + param_idx_offset, 8), 	
		doubleholdrotate= 	(EQ_PARAM_BANDORDER_OFFSET + param_idx_offset, 7), 
	)
	PARAM_EQ_2_MAPPING = PARAM_EQ_2_MAPPING + ((encoder_i_gesturemapping, ))


PARAM_EQ_2_MAPPING = EncoderMapping(*PARAM_EQ_2_MAPPING)


def HandleKnob(ID, Data2):
	mixer.automateEvent(ID, Data2, REC_MIDIController, 0, 1, 1/64)
		


MAPPINGS = {
	DELAY_3: DELAY_3_MAPPING,
	PARAM_EQ_2: PARAM_EQ_2_MAPPING
}

# Limits value between mini and maxi
def minmax(val, mini=0, maxi=1):
	return min(maxi, max(val,mini))


class TPan():
	altPressed = False
	BUTTON_GESTURE_OFFSET = 4
	ALT_BUTTON_IDX = 5
	def __init__(self):
		self.lastStepTimes = [0,0,0,0,0]
		self.wasRotated = [False,False,False,False,False]
		self.isButtonPressed = [False,False,False,False,False]

		pass
	def OnInit(event):
		pass

	def GetEncoderGestureName(self, gestureIdx):
		if gestureIdx == None:
			return None
		names = ["rotate", "doublethenrotate", "holdrotate", "doubleholdrotate", "buttonup", "buttondown"]
		res = names[gestureIdx]
		if(self.altPressed) and (gestureIdx == 0 or gestureIdx == 2): 
			res = "alt" + res
		return res

	def OnMidiMsg(self, event):
		effectName = ui.getFocusedPluginName()
		effectIndex = mixer.getActiveEffectIndex() # (mixer channel, slot)

		mapping = MAPPINGS[effectName] if effectName in MAPPINGS else None

		gesture_idx = None
		controller_idx = None
		controller_data = 0
		
		# Handle buttons only if encoders were not rotated while pressing
		if event.midiId == midi.MIDI_NOTEON: 
			buttonIdx = event.data1
			event.handled = True
			
			if buttonIdx == Pan.ALT_BUTTON_IDX:
				Pan.altPressed = True
			else:
				self.isButtonPressed[buttonIdx] = True
				gesture_idx = Pan.BUTTON_GESTURE_OFFSET + 1
				controller_idx = buttonIdx
				controller_data = 1
			
			if mapping == None:
				device.processMIDICC(event)
				return


		elif event.midiId == midi.MIDI_NOTEOFF:
			buttonIdx = event.data1
			event.handled = True

			if buttonIdx == Pan.ALT_BUTTON_IDX:
				Pan.altPressed = False
			else:
				self.isButtonPressed[buttonIdx] = False

				if not self.wasRotated[buttonIdx]:
					# handle button release
					gesture_idx = Pan.BUTTON_GESTURE_OFFSET + 0
					controller_idx = buttonIdx
					controller_data = 0

				self.wasRotated[buttonIdx] = False

				if mapping == None:
					device.processMIDICC(event)
					return

		elif event.midiId == midi.MIDI_CONTROLCHANGE:
			# Let FL Studio handle potentiometers 
			if event.data1 >= POT_BASE_IDX and event.data1 < POT_BASE_IDX + 5: 
				event.handled = False
				return
			

			
			# Get encoder idx
			encoder_idx = (event.data1 - ENCODER_BASE_IDX) % 5 
			controller_idx = encoder_idx

			encoder_gesture_idx = int((event.data1 - ENCODER_BASE_IDX) / 5) 
			gesture_idx = encoder_gesture_idx
			velocity = event.data2 - 64

			controller_data = velocity
			
			if self.isButtonPressed[encoder_idx]:
				self.wasRotated[encoder_idx] = True


			if mapping == None: 
				port = device.getPortNumber()
				#controlId = midi.EncodeRemoteControlID(port, 0, 0)
				# 0x0e : rotation (pan.js)
				controlId = ENCODER_BASE_IDX + 5 * encoder_gesture_idx + encoder_idx + (0 << 16) + ((port + 1) << 22)
				eventId = device.findEventID(controlId)
				
				#print(device.getLinkedParamName(eventId))
				event.handled = True
				newVal = channels.incEventValue(eventId, velocity, 1/64)
				general.processRECEvent(eventId, newVal, REC_Control | REC_UpdateControl)
				return
		

			

		gesture_name = self.GetEncoderGestureName(gesture_idx) 
		if gesture_name == None:
			event.handled = True
			return

		
		controlled_param = mapping[controller_idx][gesture_name]
		
		# Finally setting the parameter value	
		currentValue = plugins.getParamValue(controlled_param.idx, *effectIndex)
		change_value = controlled_param.CalculateStep(controller_data)
		plugins.setParamValue(minmax(currentValue + change_value), controlled_param.idx, *effectIndex)
		event.handled = True


Pan = TPan()
def OnInit():
	Pan.OnInit()

def OnMidiMsg(event):
	Pan.OnMidiMsg(event)
	

def OnDeInit():
	Pan.OnDeInit()