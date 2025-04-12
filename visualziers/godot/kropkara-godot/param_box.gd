extends PanelContainer


@onready var sprinbox: SpinBox = %SpinBox
@onready var label: Label = %Label



signal value_changed(value: int)
var _value: float = 0.0



static func create(name: String, value: int, min =0 , max = 100) -> Node:
    var scene = load("res://param_box.tscn").instantiate()
    scene.name = name
    scene._value = value
    scene.sprinbox.min_value = min
    scene.sprinbox.max_value = max
    scene.sprinbox.value = value
    scene.label.text = name
    return scene


func _on_value_changed(value: int) -> void:
    _value = value
    emit_signal("value_changed", value)

func _ready() -> void:
    sprinbox.value_changed.connect(_on_value_changed)
