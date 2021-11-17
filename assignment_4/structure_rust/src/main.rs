use log::*;
#[derive(Debug)]

// Student Structure
struct Student {
    name: String,
    roll_no: i32,
    score_of_each_subject: Score,
    department: String,
    school: String,
}

#[derive(Debug)]
// Score Structure
struct Score {
    hindi: f32,
    english: f32,
    maths: f32,
    science: f32,
}

// Function "new" initializes Student objects
//
// #Arguments
//
// Student Structure
//
// #Return
//
// Return the Student type objects

fn new(
    name: String,
    roll_no: i32,
    score_of_each_subject: Score,
    department: String,
    school: String,
) -> Student {
    Student {
        name,
        roll_no,
        score_of_each_subject,
        department,
        school,
    }
}

// Function "get_average" is to get average of all scores
//
// #Arguments
//
// Score Structure
//
// #Return
//
// Return the average of marks
fn get_average(average: &Score) -> f32 {
    info!("In 'get_average' function");
    let avg: f32 = (average.hindi + average.english + average.maths + average.science) / 4.0;
    avg
}

// Function "pass_student" add numbers to student's score if score < 35
//
// #Arguments
//
// Score Structure
//
// #Return
//
// Return the array of new scores stored in it
fn pass_student(marks: &Score) -> [f32; 4] {
    info!("In 'pass_student' function");
    let mut arr: [f32; 4] = [marks.hindi, marks.english, marks.maths, marks.science];

    let diff1: f32;
    let diff2: f32;
    let diff3: f32;
    let diff4: f32;
    if arr[0] < 35.0 {
        diff1 = 35.0 - arr[0]; // Stores subtracted score from 35
        arr[0] += diff1;
    }
    if arr[1] < 35.0 {
        diff2 = 35.0 - arr[1];
        arr[1] += diff2;
    }
    if arr[2] < 35.0 {
        diff3 = 35.0 - arr[2];
        arr[2] += diff3;
    }
    if arr[3] < 35.0 {
        diff4 = 35.0 - arr[3];
        arr[3] += diff4;
    }
    arr
}

// Implementing Score Structure
impl Score {
    // Function "compare_student" is to print difference of each subject's score b/w students
    //
    // #Arguments
    //
    // Score Structure
    //
    // #Return
    //
    // No Return
    fn compare_student(&self, other: &Score) {
        info!("In 'compare_student' function");
        if self.hindi >= other.hindi {
            debug!(
                "Student_1 has higher marks in Hindi by: {}",
                self.hindi - other.hindi
            );
        } else {
            debug!(
                "Student_2 has higher marks in Hindi by: {}",
                other.hindi - self.hindi
            );
        }

        if self.english >= other.english {
            debug!(
                "Student_1 has higher marks in English by: {}",
                self.english - other.english
            );
        } else {
            debug!(
                "Student_2 has higher marks in English by: {}",
                other.english - self.english
            );
        }

        if self.maths >= other.maths {
            debug!(
                "Student_1 has higher marks in Maths by: {}",
                self.maths - other.maths
            );
        } else {
            debug!(
                "Student_2 has higher marks in Maths by: {}",
                other.maths - self.maths
            );
        }

        if self.science >= other.science {
            debug!(
                "Student_1 has higher marks in Science by: {}",
                self.science - other.science
            );
        } else {
            debug!(
                "Student_2 has higher marks in Science by: {}",
                other.science - self.science
            );
        }
    }
}

// Main function
fn main() {
    env_logger::init();
    info!("Structures in Rust");
    // Specifying scores of student1
    let mut student1_score = Score {
        hindi: 32.0,
        english: 72.0,
        maths: 88.0,
        science: 83.0,
    };

    // Specifying scores of student2
    let mut student2_score = Score {
        hindi: 18.0,
        english: 68.0,
        maths: 22.0,
        science: 35.0,
    };

    // Storing present scores of student1 to arr1
    let mut arr1 = [
        student1_score.hindi,
        student1_score.english,
        student1_score.maths,
        student1_score.science,
    ];

    // Storing present scores of student2 to arr2
    let mut arr2 = [
        student2_score.hindi,
        student2_score.english,
        student2_score.maths,
        student2_score.science,
    ];
    debug!("Present scores of Student_1: {:?}", arr1);
    debug!("Present scores of Student_2: {:?}", arr2);

    debug!("\n");
    debug!("Average of Student_1: {}", get_average(&student1_score));
    debug!("Average of Student_2: {}", get_average(&student2_score));

    // Replacing arr1 with new scores of student1
    arr1 = pass_student(&student1_score);

    // Replacing arr2 with new scores of student2
    arr2 = pass_student(&student2_score);
    debug!("\n");
    debug!("New scores of Student_1: {:?}", arr1);
    debug!("New scores of Student_2: {:?}", arr2);

    // Specifying new scores to student1
    student1_score = Score {
        hindi: arr1[0],
        english: arr1[1],
        maths: arr1[2],
        science: arr1[3],
    };

    // Specifying new scores to student2
    student2_score = Score {
        hindi: arr2[0],
        english: arr2[1],
        maths: arr2[2],
        science: arr2[3],
    };

    debug!("\n");
    debug!("New Average of Student_1: {}", get_average(&student1_score));
    debug!("New Average of Student_2: {}", get_average(&student2_score));

    debug!("\n");
    student1_score.compare_student(&student2_score);

    debug!("\n");
    debug!(
        "Student_1: {:?}",
        new(
            String::from("Sandeep"),
            44,
            student1_score,
            String::from("CSE"),
            String::from("AEC")
        )
    );
    debug!(
        "Student_2: {:?}",
        new(
            String::from("Suman"),
            38,
            student2_score,
            String::from("CSE"),
            String::from("KIIT")
        )
    );
}