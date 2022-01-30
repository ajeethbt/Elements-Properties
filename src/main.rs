
use std::io::stdin;
use std::io::BufRead;

fn compileshow() {
   println!("Compiling please wait...");   
}

fn main() {
    compileshow();
    let mut atomnumb = String::new();
    //let val = String::new();
    let mut atomname = String::new();
    let exitstat:bool = false; 
    let mut inval = String::new();


      println!("");
      println!("");
      println!("    ________                          __     ____          __         ");              
      println!("   / ____/ /__  ____ ___  ___  ____  / /_   /  _/___  ____/ /__  _  __");                 
      println!("  / __/ / / _ \\/ __ `__ \\/ _ \\/ __ \\/ __/   / // __ \\/ __  / _ \\| |/_/");                
      println!(" / /___/ /  __/ / / / / /  __/ / / / /_   _/ // / / / /_/ /  __/>  <");                 
      println!("/_____/_/\\___/_/ /_/ /_/\\___/_/ /_/\\__/  /___/_/ /_/\\__,_/\\___/_/|_|");               
                                                                      










    
    let elements = ["Please"," Hydrogen"," Helium","Lithium"," Beryllium","Boron","Carbon"," Nitrogen","Oxygen","Fluorine","Neon","Sodium","Magnesium","Aluminium","Silicon","Phosphorus","Sulfur"," Chlorine","Argon","Potassium","Calcium "," Scandium","Titanium","Vanadium",
                    "Chromium","Manganese"," Iron","Cobalt","Nickel","Copper","Zinc","Gallium","Germanium","Arsenic"," Selenium","Bromine","Krypton","Rubidium","Strontium","  Yttrium","Zirconium ","Niobium ","Molybdenum ","Technetium ","Ruthenium ","Rhodium","Palladium","Silver",
                    "Cadmium "," Indium ","Tin","Antimony"," Tellurium","Iodine","Xenon","Cesium","Barium","Lanthanum","Cerium"," Praseodymium","Neodymium"," Promethium "," Samarium ","Europium "," Gadolinium","Terbium "," Dysprosium ","Holmium "," Erbium","Thulium","Ytterbium","Lutetium","Hafnium",
                    "Tantalum "," Tungsten","Rhenium ","Osmium ","Iridium ","Platinum","Gold","Mercury"," Thallium","Lead ","Bismuth"," Polonium" ,"Astatine","Radon ","Francium ","Radium"," Actinium","Thorium"," Protactinium","Uranium","Neptunium","Plutonium","Americium","Curium","Berkelium","Californium"];
    let elesymb = [" ","H","He","Li","Be","B","C","N","O","F","Ne","Na","Mg","Al","Si","P","S","Cl","K","Ar","Ca","Sc","Ti","V","Cr","Mn","Fe","Ni","Co","Cu","Zn","Ga","Ge","As","Se","Be","Kr","Rb","Sr","Y","Zr","Nb","Mo","Tc","Ru","Rh","Pd","Ag","Cd","In","Sn","Sb","I","Te","Xe","Cs","Ba",
                   "La","Ce","Pr","Nd","Pm","Sm","Eu","Gd","Tb","Dy","Ho","Er","Tm","Yb","Lu","Hf","Ta","W","Re","Os","Ir","Pt","Au","Hg","Tl","Pb","Bi","Po","At"];
    
                   
    let elements = elements.map(str::trim);
   
    

    while exitstat != true {
            
         println!("\n\rWelcome to Periodic Index Please Choose the following options below: \n\r Get the name and symbol of the element from atomic number: 1 \n\r Get the atomic number and name from the element name: 2 \n\r Get the atomic number from the element symbol: 3 \n\r ");
         stdin().read_line(&mut inval).expect("Please provide a proper input,(Numbers only)");
         let inval:u8 = inval.trim().parse().expect("Error");
         
         if inval == 1 {
            println!("Please enter the atomic number: ");
            stdin().read_line(&mut atomnumb).expect("Please provide a proper input,(Numbers only)");
            let atomnumb:usize = atomnumb.trim().parse().expect("Please provide a proper input(Numbers only)");
            if atomnumb == 0 {
                println!("Please start with 1");
            } else {
                println!("This is {} with the atomic number {} and symbol {}",elements[atomnumb],atomnumb,elesymb[atomnumb]);
            }

         } else if inval == 2 {
             let mut found:bool = false;
             println!("Please enter the atomic number: ");
             stdin().read_line(&mut atomname).expect("Please provide a proper input,(Numbers only)");
             println!("{}",atomname);
             let mut x:usize = 1;
          while found == false {
               if atomname == elements[x]  {
                   found = true;
                   let whichone = x;
                   println!("this is {}",elements[whichone]);
               }else {
                   x = x+1;
               }
             
          }
               
         } else if inval == 3 {


         } else {


         }

    }

  }


   
   
   
    