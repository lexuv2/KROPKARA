extends Sprite2D


var dragging = false
var drag_offset = Vector2.ZERO

func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.is_pressed():
			var rect = get_rect()
			if rect.has_point(event.position - global_position):
				drag_offset = position - event.position
				# position = event.position
				
				print_debug(drag_offset)
				dragging = true
				get_viewport().set_input_as_handled()
		else:
			dragging = false
		


	elif event is InputEventMouseMotion and dragging:
		position = event.position + drag_offset
		get_viewport().set_input_as_handled()
	
