extends Control
class_name DragableTexture;


var dragging = false
var drag_offset = Vector2.ZERO

@onready var texture_rect:TextureRect =%main_map
@onready var container: PanelContainer = %PanelContainer
@onready var desc_label: Label = %description
static var last_selectd: DragableTexture = null;

func set_texture(tex: Texture2D):
	texture_rect.texture = tex

func set_description(desc: String):
	desc_label.text = desc

func _ready() -> void:
	container.gui_input.connect(container_gui_input)


func container_gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.is_pressed() and not dragging:
			#var rect = get_rect()
			#if rect.has_point(event.position - global_position):
			drag_offset = position - event.position
				# position = event.position
				
			#print_debug(drag_offset)
			dragging = true
			
			#get_viewport().set_input_as_handled()
		else:
			dragging = false
		


	elif event is InputEventMouseMotion and dragging:
		#print(event.position)
		#position = event.position + drag_offset
		position += event.relative
		#get_viewport().set_input_as_handled()
	


func _process(delta: float) -> void:
	if dragging:
		last_selectd = self
	if dragging and Input.is_action_just_pressed("delete_map"):
		queue_free()
