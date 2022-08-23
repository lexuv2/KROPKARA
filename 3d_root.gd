extends Spatial


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass

const HTerrainData = preload("res://addons/zylann.hterrain/hterrain_data.gd")

func load_map(path):
	var map_base_num
	var map_mid_num
	var map_top_num
	var map = []
	var file = File.new()
	var xx =0
	var yy =0
	
	var terrain_data = HTerrainData.new()
	terrain_data.resize(513)
	
	
	file.open(path,file.READ)
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
	
	var img = Image.new()
	var deb_img: Image = terrain_data.get_image(HTerrainData.CHANNEL_HEIGHT)
	deb_img.create(xx, yy, false, Image.FORMAT_RGBA8)
	deb_img.lock()
	
	for i in yy:
		for j in xx:
			var br = float(map[i][j]);
			#print(br)
			
			#var r = 561*br - 306*br*br;
			#var g = 204*br*br - 102*br + 153;
			#var b = 632*br*br-404*br+51;
			#r/=255;
			#g/=255;
			#b/=255;
			#print(br, " " , " ",r," ",g," ",b)
			

			

			
			var debr = (br-map_base_num)/(map_top_num-map_base_num)
			deb_img.set_pixelv(Vector2(i,j),Color(debr,debr,debr))
			#img.fill_rect(Rect2(i,j,1,1) ,inter)
	deb_img.unlock()
	deb_img.save_png("deb.png")

	#var modified_region = Rect2(Vector2(), deb_img.get_size())
	#terrain_data.notify_region_change(modified_region, HTerrainData.CHANNEL_HEIGHT)
	var data = $WorldEnvironment/HTerrain.get_data()
	var params={}
	params[HTerrainData.CHANNEL_HEIGHT] = {
			"path": "deb.png",
			"min_height": 0,
			"max_height": 100,
			"big_endian": 1
		}
	data._edit_import_maps(params)
	#$WorldEnvironment/HTerrain.set_shader_type($WorldEnvironment/HTerrain.SHADER_LOW_POLY)
	
	pass
func _on_FileDialog_file_selected(path):
	load_map(path)
	pass # Replace with function body.


func _on_Button_pressed():
	$FileDialog.popup()
	pass # Replace with function body.
