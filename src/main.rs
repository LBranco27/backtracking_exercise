use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Result, Write};
use rand::Rng;

fn allocate_time(input_file: &str, output_file: &str) -> Result<()> {
    let mut machines: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    //println!("testes");

    // read
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    let mut all_bytes: Vec<Vec<u8>> = Vec::new();
    let mut read_index = 0;
    all_bytes.push(Vec::new());
    for byte_result in reader.bytes() {
        let byte = byte_result?;
        if byte == b'\n' {
            read_index += 1;
            all_bytes.push(Vec::new());
            continue;
        }
        all_bytes[read_index].push(byte);
    }
    for line in all_bytes {
        if !line.is_empty(){
            lines.push(String::from_utf8_lossy(&line).to_string());
        }
    }
//    for line in lines {
//        println!("{line}");
//    }
//    for line in reader.lines() {
//        println!("TESTE");
//        println!("{:?}", String::from(line?));
//    }
//    let file = File::open(input_file)?;
//    let reader = BufReader::new(file);
    for line in lines {
        println!("TESTE");
        //let line = String::from(line?);
        println!("TESTE");
        let parts: Vec<&str> = line.split(':').collect();
        let machine = parts[0].to_string();
        let students_str = parts[1].trim();
        //println!("{students_str}");
        let students: Vec<(String, usize)> = students_str
            .split(';')
            .filter(|s| !s.is_empty())
            .map(|student| {
                let parts: Vec<&str> = student.split('=').collect();
                (parts[0].to_string(), parts[1].parse().unwrap())
            })
            .collect();
        machines.insert(machine, students);
    }
    
    // print
    for (machine, students) in &machines {
        print!("!!!{machine}:!!! ");
        for student in students {
            print!("{}={} ", student.0, student.1);
        }
    }
    println!("\n\n\n");

    // vector allocated students
    //let mut allocated_students: HashSet<String> = HashSet::new();
    let mut current_allocations: Vec<String> = Vec::new();


    // vector machine allocations
    let mut machine_allocations: Vec<Vec<String>> = Vec::new();
    for _ in 0..12 {
        machine_allocations.push(Vec::new());
    }

//    // backtrack function
//    fn backtrack(
//        machines: &mut HashMap<String, Vec<(String, usize)>>,
//        machine_allocations: &mut Vec<Vec<String>>,
//        current_allocations: &mut Vec<String>,
//        index: usize,
//    ) -> bool {
//        if index == 11 {
//            // allocation finished
//            return true;
//        }
//        
//        let mut rng = rand::thread_rng();
//        for (machine, students) in machines.iter_mut() {
//            let mut teste = 0;
//            let random_student_index = rng.gen_range(0..students.len());
//            let (student, count) = students[random_student_index].clone();
//            let mut student_name = student;
//            let mut student_count = count;
//            //println!("\n{}", student_name);
//            while current_allocations.contains(&student_name){
//                println!("student already added");
//                let random_student_index = rng.gen_range(0..students.len());
//                let (student, count) = students[random_student_index].clone();
//                student_name = student;
//                student_count = count;
//                teste += 1;
//                if teste == 20{
//                    return false;
//                }
//            }
//            current_allocations.push(student_name.clone());
//            let new_count = student_count - 1;
//            if new_count == 0{
//                students.remove(random_student_index);
//            } else {
//                students[random_student_index] = (student_name, new_count);
//            }
//        }
//        machine_allocations[index] = current_allocations.clone();
//        current_allocations.clear();
//        println!("PAROU");
//        if backtrack(machines, machine_allocations, current_allocations, index + 1) {
//            println!("machine");
//            return true;
//        }
//        false
//    }

    // backtrack function
    fn backtrack(
        machines: &mut HashMap<String, Vec<(String, usize)>>,
        machine_allocations: &mut Vec<Vec<String>>,
        current_allocations: &mut Vec<String>,
        machines_index: Vec<String>,
        index: &mut usize,
    ) -> bool {
        //println!("\ninicio");
        //println!("{index}");
        // verificar qual falta no current_allocations e colocar um estudante que esteja
        // interessado nele. incrementar o current_allocations e realizar backtrack.
        // 1. caso current_allocations esteja cheio, passar tudo para machine_allocations
        // e + 1 no index e limpar current_allocations.
        // 2. caso backtrack retornou falso, entao jogar estudante atual para a
        // lista de estudantes ja verificados e tentar outros. Caso nao tenha 
        // nenhum que funcione, retornar false.

//        let mut current_index = 0;
//        if current_allocations.len() >= 1{
        let mut current_index = current_allocations.len();
//        }
        //println!("cur: {}", current_allocations.len());

        // reset
        if current_allocations.len() == machines.len(){
            machine_allocations[*index] = current_allocations.clone();
            println!("machine_allocation[{index} = {:?}", machine_allocations[*index]);
            current_allocations.clear();
            current_index = 0;
            *index += 1;
        }
        if *index == 12 {
            // allocation finished
            return true;
        }

        let mut tested_students: Vec<String> = Vec::new();

        let mut rng = rand::thread_rng();
        let machine_clone = machines.clone();
        while 1 == 1 { //aqui poderia ter uma verificacao se todos os alunos ja foram testados
            println!("CURRENT INDEX: {current_index}");
            *machines = machine_clone.clone();
            let current_students = machines.get_mut(&machines_index[current_index]);
            let current_students = match current_students {
                Some(students) => students,
                None => return false,
            };
            println!("STUDENTS BEFORE: {:?}", current_students);
            let random_student_index = rng.gen_range(0..current_students.len());
            let (student, _count) = &mut current_students[random_student_index].clone();
            let mut check = 0;
            println!("STUDENT CHOSEN: {}", student);
            println!("CURRENT ALLOCATIONS: {:?}", current_allocations);
            if current_allocations.contains(student){
                println!("STUDENT ALREADY ON CURRENT_ALLOCATIONS, RETURNING FALSE");
                return false;
            }
            for (student, _) in &mut *current_students{
                if !tested_students.contains(student){
                    check = 1;
                }
            }
            if check == 0{
                return false;
            }

            current_allocations.push(student.clone());
            tested_students.push(student.clone());
            println!("TESTED STUDENTS: {:?}", tested_students);
            let (_, count_true) = &mut current_students[random_student_index]; //resenha
            *count_true -= 1;
            if *count_true == 0 {
                current_students.remove(random_student_index);
            }
            println!("STUDENTS AFTER: {:?}", current_students);

            if backtrack(machines, machine_allocations, current_allocations, machines_index.clone(), index){
                return true;
            }

            println!("RESETING TO TRY AGAIN!");
            current_allocations.pop();
        }
        false
    }

    // run backtrack. write output
    let mut index = 0;
    let mut machines_index: Vec<String> = Vec::new();
    for (machine, _) in &machines{
        machines_index.push(machine.clone());
    }
    if backtrack(&mut machines, &mut machine_allocations, &mut current_allocations, machines_index.clone(), &mut index) {
        let mut output_file = File::create(output_file)?;

        for aaaa_index in 0..machines.len(){
            write!(output_file, "{}:", machines_index.clone()[aaaa_index])?;
            for mach_index in 0..machine_allocations.len(){
                //println!("{mach_index} and {aaaa_index}");
                write!(output_file, "-{}", machine_allocations[mach_index][aaaa_index])?;
            }
            writeln!(output_file)?;
        }

        println!("output_file written!");
        Ok(())
    } else {
        println!("error.");
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "not possible to allocate.",
        ))
    }
}

fn main() {
    let input_file = "entrada_6.txt";
    let output_file = "output.txt";
    if let Err(e) = allocate_time(input_file, output_file){
        eprintln!("{}", e);
    }
}
