
pub struct Dj {
    pub(crate) name: String,
    pub(crate) music_genre: Vec<String>
}

trait Visible {
    fn draw(&self, canvas: &mut u32);
    fn hit_test(&self, x: i32, y: i32) -> bool;
}