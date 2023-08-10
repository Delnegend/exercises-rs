mod house_items;
use house_items::properties::KíchThước;
use house_items::properties::KiểuDáng;
fn main() {
    let cửa = house_items::Door::new("wood", KíchThước::BìnhThường);
    println!("The {} door's size is {}", cửa.material(), cửa.size());
    let cửa_sổ = house_items::Window::new("wood", KíchThước::To, KiểuDáng::HiệnĐại);
    println!(
        "The {} window is {} and {}",
        cửa_sổ.material(),
        cửa_sổ.size(),
        cửa_sổ.style()
    );
}