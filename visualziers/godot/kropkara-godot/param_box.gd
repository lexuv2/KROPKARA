extends PanelContainer
class_name ParamBox

@onready var spinbox: SpinBox = %SpinBox
@onready var label: Label = %Label



signal value_changed(value: int)
var value: float = 0.0



static func create(creator: Node,name: String, value: int, min =0 , max = 100,step =1 ) -> PanelContainer:
	var scene = preload("res://param_box.tscn").instantiate()
	creator.add_child(scene)
	scene.name = name
	scene.value = value
	scene.spinbox.min_value = min
	scene.spinbox.max_value = max
	scene.spinbox.value = value
	scene.spinbox.step = step
	
	scene.label.text = name
	return scene


func _on_value_changed(value) -> void:
	self.value = value
	print_debug(value)
	emit_signal("value_changed", value)

func _ready() -> void:
	spinbox.value_changed.connect(_on_value_changed)
