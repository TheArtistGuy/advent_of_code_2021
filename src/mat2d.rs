#[derive(Clone)]
pub struct Mat2d<T>{
    pub height: usize,
    pub width: usize,
    pub vector : Vec<T>
}

impl<T> Mat2d<T>{
    /**
    fn new(width : usize, height : usize) ->Self{
        Board{
            vector: vec![T; (width*height) ],
            width,
            height
        }
    }
**/
    pub fn from(vector : Vec<T>, width: usize, height: usize)-> Self{
        assert_eq!(vector.len(), (width * height ));
        Mat2d{
            vector,
            width,
            height
        }
    }

    pub fn get_field(&self, col : usize, row : usize) -> Option<&T> {
        if col >= self.width || row >= self.height{
            return None;
        }
        self.vector.get(col + (self.width * row))
    }

    pub fn set_field(&mut self, col : usize, row : usize, value : T) -> Result<bool, &str> {
        if col >= self.width || row >= self.height{
            return Err("Not in matrix");
        }
        self.vector.remove(col + (self.width * row));
        self.vector.insert(col + (self.width * row), value);
        Ok(true)
    }

}
