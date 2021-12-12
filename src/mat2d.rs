///A Datastruture for a 2 dimensional Matrix
///
/// Cloneable
#[derive(Clone)]
pub struct Mat2d<T>{
    pub height: usize,
    pub width: usize,
    pub vector : Vec<T>
}

impl<T> Mat2d<T>{
    ///converts a vector of size (width * heigth) into a Matrix with
    /// width colums and height rows
    pub fn from(vector : Vec<T>, width: usize, height: usize)-> Self{
        assert_eq!(vector.len(), (width * height ));
        Mat2d{
            vector,
            width,
            height
        }
    }

    ///returns the Value in the Position specified
    pub fn get_value(&self, col : usize, row : usize) -> Option<&T> {
        if col >= self.width || row >= self.height{
            return None;
        }
        self.vector.get(col + (self.width * row))
    }

    ///returns the Value in the Position specified, also takes negative numbers, but returns None
    ///
    /// Convenient if you want to find neighbours.
    pub fn get_value_in_position_of_i32(&self, col : i32, row : i32) -> Option<&T> {
        if col<0 || row<0 || col as usize>= self.width || row as usize >= self.height{
            return None;
        }
        self.vector.get(col as usize+ (self.width * row as usize))
    }

    ///Sets the value in teh especified position to value
    pub fn set_value(&mut self, col : usize, row : usize, value : T) -> Result<bool, &str> {
        if col >= self.width || row >= self.height {
            return Err("Not in matrix");
        }
        self.vector.remove(col + (self.width * row));
        self.vector.insert(col + (self.width * row), value);
        Ok(true)
    }
}
