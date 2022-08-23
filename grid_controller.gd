extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var grid=[];

var pt_scene = preload("res://point.tscn")
var last_file = ""
var gap = 1

# Called when the node enters the scene tree for the first time.
export var siz =300;


var map_base_num =0
var map_mid_num =0
var map_top_num=0


var xx = 0
var yy = 0

export var br =0
var map=[]
var ui = "../Panel/"


func _ready():

		
	#print(map)
	pass # Replace with function body.

func _unhandled_input(event):
	var pos = get_local_mouse_position();
	if event is InputEventMouseMotion and Input.is_action_pressed("l_click") and pos.x > 0 and pos.x < xx and pos.y >0 and pos.y < yy:
		var zv = Vector2(-scale.x,-scale.y)
		position+=event.relative#*zv
	
	
func _input(event):
	var pos = get_local_mouse_position();
	if event is InputEventMouseButton and pos.x > 0 and pos.x < xx and pos.y >0 and pos.y < yy:
		if event.button_index == BUTTON_WHEEL_UP and event.pressed:
			scale.x+=0.05
			scale.y+=0.05
			
		if event.button_index == BUTTON_WHEEL_DOWN and event.pressed:
			scale.x-=0.05
			scale.y-=0.05
		
		
		
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	var pos = get_local_mouse_position();
	if pos.x > 0 and pos.x < xx and pos.y >0 and pos.y < yy:
		get_node(ui+"ColorRect").color=Color(0.5,0,0)
	else:
		get_node(ui+"ColorRect").color=Color("25252a")
	#$height_indicator.position  = get_local_mouse_position()
	#var pos = get_local_mouse_position();
	if pos.x > 0 and pos.x < xx and pos.y >0 and pos.y < yy:
		get_node("../height_indicator/Label").text = map[int(get_local_mouse_position().x)][int(get_local_mouse_position().y)];
		get_node(ui+"coords_indciator").text="X: "+str(int(pos.x))+" Y: "+str(int(pos.y));
	else:
		get_node("../height_indicator/Label").text=""
	pass




func update_map():
	
	#update()
	#return
	var img = Image.new()
	img.create(xx, yy, false, Image.FORMAT_RGBA8)
	img.lock()
	
	var deb_img = Image.new()
	deb_img.create(xx, yy, false, Image.FORMAT_RGBA8)
	deb_img.lock()

	for i in yy:
		for j in xx:
			br = float(map[i][j]);
			#print(br)
			
			#var r = 561*br - 306*br*br;
			#var g = 204*br*br - 102*br + 153;
			#var b = 632*br*br-404*br+51;
			#r/=255;
			#g/=255;
			#b/=255;
			#print(br, " " , " ",r," ",g," ",b)
			
			var base =get_node(ui+"color_base").color
			var mid = get_node(ui+"color_mid").color
			var top = get_node(ui+"color_top").color
			
			var inter = Color()
			if (br<map_mid_num):
				inter = base.linear_interpolate(mid,(br-map_base_num)/(map_mid_num-map_base_num));
			else:
				inter = mid.linear_interpolate(top,(br-map_mid_num)/(map_top_num-map_mid_num))
			img.set_pixelv(Vector2(i,j) ,inter) # Works
			
			var debr = (br-map_base_num)/(map_top_num-map_base_num)
			deb_img.set_pixelv(Vector2(i,j),Color(debr,debr,debr))
			#img.fill_rect(Rect2(i,j,1,1) ,inter)
	img.unlock()
	deb_img.unlock()
	deb_img.save_png("deb.png")
	var texture = ImageTexture.new()
	texture.create_from_image(img,2)
	$Sprite.texture=texture
	#update()

func _on_grid_controller_draw():
	#print(position)
	
	#update_map()
	return
	for i in yy:
		for j in xx:
			br = float(map[i][j]);
			#print(br)
			
			#var r = 561*br - 306*br*br;
			#var g = 204*br*br - 102*br + 153;
			#var b = 632*br*br-404*br+51;
			#r/=255;
			#g/=255;
			#b/=255;
			#print(br, " " , " ",r," ",g," ",b)
			
			var base =get_node(ui+"color_base").color
			var mid = get_node(ui+"color_mid").color
			var top = get_node(ui+"color_top").color
			
			var inter = Color()
			if (br<map_mid_num):
				inter = base.linear_interpolate(mid,(br-map_base_num)/(map_mid_num-map_base_num));
			else:
				inter = mid.linear_interpolate(top,(br-map_mid_num)/(map_top_num-map_mid_num))
			#inter/=255;
			draw_rect(Rect2(i,j,1,1) ,inter)
	pass # Replace with function body.

func load_file():
	map = []
	update_map()
	var file = File.new()
	file.open(last_file,file.READ)
	var fir = true
	while !file.eof_reached():
		var line = file.get_csv_line(";")
		if fir:
			xx = int(line[0])
			yy = int(line[1])
			fir = false
			continue
		
		map.append(line);
	map_base_num=9999999;
	map_top_num=0;
	for i in yy:
		if i < yy/5 or i>yy-(xx/5):
			continue;
		for j in xx:

			if j < xx/5 or j>xx-(xx/5):
				continue;
			map_base_num = min(map_base_num,float(map[i][j]));
			map_top_num = max(map_top_num,float(map[i][j]));
	map_mid_num=(map_base_num+map_top_num)/2
	get_node(ui+"map_base_num").text=str(int(map_base_num))
	get_node(ui+"map_mid_num").text=str(int(map_mid_num))
	get_node(ui+"map_top_num").text=str(int(map_top_num))
	if map_base_num==0:
		map_base_num=0.1
	if map_mid_num==0:
		map_mid_num=0.1
	update_map()

func _on_FileDialog_file_selected(path):
	last_file=path
	load_file()
	pass # Replace with function body.


func _on_load_button_pressed():
	load_file()
	pass # Replace with function body.





func _on_color_base_color_changed(color):
	update_map()
	pass # Replace with function body.
func _on_color_mid_color_changed(color):
	update_map()
	pass # Replace with function body.
func _on_color_top_color_changed(color):
	update_map()
	pass # Replace with function body.


func _on_map_base_num_text_changed(new_text):
	map_base_num=int(new_text)
	update_map()
	pass # Replace with function body.


func _on_map_mid_num_text_changed(new_text):
	map_mid_num=int(new_text)
	update_map()
	pass # Replace with function body.


func _on_map_top_num_text_changed(new_text):
	map_top_num=int(new_text)
	update_map()
	pass # Replace with function body.
