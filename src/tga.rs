use std::{fs::File, io::{BufReader, Read}, mem, path::Path};
#[derive(Clone)]
pub struct Tga {
    pub map: Vec<Vec<u32>>,
    width: usize,
    height: usize
}

impl Tga {
    pub fn read_file(path: &Path) -> Tga{
        let mut file = BufReader::new(File::open(path).unwrap());

        let mut header_bytes: [u8; HEADERSIZE] = [0; HEADERSIZE];

        file.read_exact(&mut header_bytes).unwrap();
        let header = unsafe {
            mem::transmute::<[u8; HEADERSIZE], TgaHeader>(header_bytes)
        };        


        let width = header.width as usize;
        let height = header.height as usize;

        let bytespp = (header.bitsperpixel>>3) as usize;

        let mut buffer: Vec<u8> = Vec::with_capacity(width * height * bytespp);

        if 3==header.datatypecode || 2==header.datatypecode{ 
            file.read_to_end(&mut buffer).unwrap();
        } else if 10==header.datatypecode || 11==header.datatypecode {
            Tga::read_rle(width*height, bytespp, &mut file, &mut buffer)    
        }

        

        let mut map = Tga::create_canvas(width, height, bytespp, &buffer);
    	if header.imagedescriptor&0x20 > 0 {

            for iy in 0..height/2{
                for item in map.iter_mut().take(width){
                    item.swap(iy, height-1-iy);
                }
            }
        }

        Tga {
            map,
            width,
            height
        }
    }
    fn read_rle<T: Read>(pixelcount: usize, bytespp: usize, file: &mut T, encoded: &mut Vec<u8>){
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let mut pos = 0;
        let mut pix = 0;
        while pix < pixelcount {

            let mut chunkheader = buffer[pos] as usize;
            pos+=1;

            if chunkheader<128 {
                chunkheader+=1;
                let endpos = pos+chunkheader*bytespp;
                while pos < endpos {
                    encoded.push(buffer[pos]);
                    pos+=1;
                }
            } else {
                chunkheader -= 127;
                for _i in 0..chunkheader{
                    for j in 0..bytespp{
                        encoded.push(buffer[pos+j]);
                    }
                }
                pos+=bytespp;
            }
            pix+=chunkheader;
        }
    }

    fn create_canvas(width: usize, height: usize, bytespp: usize, buffer: &Vec<u8>) -> Vec<Vec<u32>>{
        let mut canvas = vec![vec![0;height];width];
        for iy in 0..height{
            for ix in 0..width{
                if bytespp == 1 {
                    let intensity = buffer[iy*width+ix] as u32;
                    canvas[ix][iy] = intensity + (intensity << 8) + (intensity << 8);
                } else if bytespp == 3 {
                    let bytes = &buffer[(iy*width+ix)*3..(iy*width+ix+1)*3];
                    canvas[ix][iy] = bytes[0] as u32 + ((bytes[1] as u32) << 8) + ((bytes[2] as u32) << (8*2));
                } else if bytespp == 4 {
                    let bytes = &buffer[(iy*width+ix)*4..(iy*width+ix+1)*4];
                    canvas[ix][iy] = bytes[0] as u32 + ((bytes[1] as u32) << 8) + ((bytes[2] as u32) << (8*2));
                }
            }
        }
        canvas
    }
    pub fn get_pixel(&self, x: i32, y: i32) -> u32 {
        self.map[x as usize][y as usize]
    }
    
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}

const HEADERSIZE: usize = 18;

#[derive(Debug)]
#[repr(C, packed)]
struct TgaHeader {
	idlength: i8,
	colormaptype: i8,
	datatypecode: i8,
	colormaporigin: i16,
	colormaplength: i16,
	colormapdepth: i8,
	x_origin: i16,
	y_origin: i16,
	width: i16,
	height: i16,
	bitsperpixel: i8,
	imagedescriptor: i8,
}