extends Camera


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

var l_mouse_pos = Vector2.ZERO
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	
	var ud = Input.get_action_strength("ui_up") - Input.get_action_strength("ui_down")
	translation-=global_transform.basis.z*ud
	
	var lr = Input.get_action_strength("ui_left") - Input.get_action_strength("ui_right")
	translation-=global_transform.basis.x*lr
	
	
	if Input.get_action_strength("l_click"):
		var pos = get_viewport().get_mouse_position()
		var del = l_mouse_pos - pos
		#print(del)
		rotation_degrees.x+=del.y
		rotation_degrees.y+=del.x
		l_mouse_pos = pos

