slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {

    let menu = Menu::new()?; 
    let photos = Photos::new()?;
    let cards = Cards::new()?;
    let game = Game::new()?;


    let mut x = String::new();
    std::io::stdin().read_line(&mut x).expect("Error");
    let x_int : i32 = x.trim().parse().expect("Error"); 

    if x_int == 1 {

        menu.run()
    } 

    else if x_int == 2{
        photos.run()
    }

    else if x_int == 3{
        cards.run()
    }

    else if x_int == 4 {
        game.run()
    }

    else{
        menu.run()
    }

}