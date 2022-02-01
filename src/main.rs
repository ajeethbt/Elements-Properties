
use std::io::stdin;

fn compileshow() {
   println!("Compiling please wait...");   
}

fn main() {
    compileshow();
    let exitstat:bool = false; 


      println!("");
      println!("");
      println!("    ________                          __     ____          __         ");              
      println!("   / ____/ /__  ____ ___  ___  ____  / /_   /  _/___  ____/ /__  _  __");                 
      println!("  / __/ / / _ \\/ __ `__ \\/ _ \\/ __ \\/ __/   / // __ \\/ __  / _ \\| |/_/");                
      println!(" / /___/ /  __/ / / / / /  __/ / / / /_   _/ // / / / /_/ /  __/>  <");                 
      println!("/_____/_/\\___/_/ /_/ /_/\\___/_/ /_/\\__/  /___/_/ /_/\\__,_/\\___/_/|_|");               
                                                                      





    
    let elements = ["Please"," hydrogen"," helium","lithium"," beryllium","boron","carbon"," nitrogen","oxygen","fluorine","neon","sodium","magnesium","aluminium","silicon","phosphorus","sulfur"," chlorine","argon","potassium","calcium "," scandium","titanium","vanadium",
                    "chromium","manganese"," iron","cobalt","nickel","copper","zinc","gallium","germanium","arsenic"," selenium","bromine","krypton","rubidium","strontium","  yttrium","zirconium ","niobium ","molybdenum ","technetium ","ruthenium ","rhodium","palladium","silver",
                    "cadmium "," indium ","tin","antimony"," tellurium","iodine","xenon","cesium","barium","lanthanum","cerium"," praseodymium","neodymium"," promethium "," samarium ","europium "," gadolinium","terbium "," dysprosium ","holmium "," erbium","thulium","ytterbium","lutetium","hafnium",
                    "tantalum "," tungsten","rhenium ","osmium ","iridium ","platinum","gold","mercury"," thallium","lead ","bismuth"," polonium" ,"astatine","radon ","francium ","radium"," actinium","thorium"," protactinium","uranium","neptunium","plutonium","americium","curium","berkelium","californium"];
    let elesymb = [" ","H","He","Li","Be","B","C","N","O","F","Ne","Na","Mg","Al","Si","P","S","Cl","K","Ar","Ca","Sc","Ti","V","Cr","Mn","Fe","Ni","Co","Cu","Zn","Ga","Ge","As","Se","Be","Kr","Rb","Sr","Y","Zr","Nb","Mo","Tc","Ru","Rh","Pd","Ag","Cd","In","Sn","Sb","I","Te","Xe","Cs","Ba",
                   "La","Ce","Pr","Nd","Pm","Sm","Eu","Gd","Tb","Dy","Ho","Er","Tm","Yb","Lu","Hf","Ta","W","Re","Os","Ir","Pt","Au","Hg","Tl","Pb","Bi","Po","At"];
    
                   
    let elements = elements.map(str::trim);
   
    

    while exitstat != true {
            
         println!("\n\rWelcome to Periodic Index Please Choose the following options below: \n\r Get the name and symbol of the element from atomic number: 1 \n\r Get the atomic number and name from the element name: 2 \n\r Get the atomic number from the element symbol: 3 \n\r ");
         let mut inval = String::new();
         stdin().read_line(&mut inval).expect("Please provide a proper input,(Numbers only)");
         let inval:u8 = inval.trim().parse().expect("Error");
         
         if inval == 1 {
            let mut atomnumb = String::new();
            println!("Please enter the atomic number: ");
            stdin().read_line(&mut atomnumb).expect("Please provide a proper input,(Numbers only)");
            let atomnumb:usize = atomnumb.trim().parse().expect("Please provide a proper input(Numbers only)");
            if atomnumb == 0 {
                println!("Please start with 1");
            } else {
                println!("This is {} with the atomic number {} and symbol {}",elements[atomnumb],atomnumb,elesymb[atomnumb]);
            }

         } else if inval == 2 {
             println!("Please enter the name of your element: ");
             let mut atomname = String::new();
             stdin().read_line(&mut atomname).expect("send helps");
             let atomname = atomname.trim_matches(' ').to_lowercase(); 
             let mut x:usize = 0;
             let mut found = false;
               while found != true {
                 if x < 90 {
                  x = x+1;
                 }
                  x = x;
                  println!("{}",elements[x]);
                  if atomname == elements[x].trim_matches(' ').to_lowercase() {
                    println!("{}",x);
                    found = true;
                  }
                  found = found;
             
               } 
              
             
        
               
         } else if inval == 3 {


         } else {


         }

    }
  
  }
  


   
   
   
    