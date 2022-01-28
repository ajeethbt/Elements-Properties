
use std::io::stdin;

fn main() {
    println!("Please enter the atomic number: ");
    let mut atomnumb = String::new();
    let elements = ["Please"," Hydrogen"," Helium","Lithium"," Beryllium","Boron","Carbon"," Nitrogen","Oxygen","Fluorine","Neon","Sodium","Magnesium","Aluminium","Silicon","Phosphorus","Sulfur"," Chlorine","Argon","Potassium","Calcium "," Scandium","Titanium","Vanadium",
                    "Chromium","Manganese"," Iron","Cobalt","Nickel","Copper","Zinc","Gallium","Germanium","Arsenic"," Selenium","Bromine","Krypton","Rubidium","Strontium","  Yttrium","Zirconium ","Niobium ","Molybdenum ","Technetium ","Ruthenium ","Rhodium","Palladium","Silver",
                    "Cadmium "," Indium ","Tin","Antimony"," Tellurium","Iodine","Xenon","Cesium","Barium","Lanthanum","Cerium"," Praseodymium","Neodymium"," Promethium "," Samarium ","Europium "," Gadolinium","Terbium "," Dysprosium ","Holmium "," Erbium","Thulium","Ytterbium","Lutetium","Hafnium",
                    "Tantalum "," Tungsten","Rhenium ","Osmium ","Iridium ","Platinum","Gold","Mercury"," Thallium","Lead ","Bismuth"," Polonium" ,"Astatine","Radon ","Francium ","Radium"," Actinium","Thorium"," Protactinium","Uranium","Neptunium","Plutonium","Americium","Curium","Berkelium","Californium"];
   let elements = elements.map(str::trim);
   stdin().read_line(&mut atomnumb).expect("Please provide a proper input,(Numbers only)");
   let atomnumb:usize = atomnumb.trim().parse().expect("Please provide a proper input(Numbers only)");
   if atomnumb == 0 {
       println!("Please start with 1.");
   }else {
    println!("This is {} with the atomic number {}",elements[atomnumb],atomnumb);
   }
  
}