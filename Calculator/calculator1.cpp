#include <iostream>
#include <cctype>
#include <cmath>
#include <math.h>
using namespace std;

////Function to add two numbers together
//double add(double num1, double num2) {
//    return num1 + num2;
//}

////Function to add multiple numbers together
//void add_input() {
//    double result = 0;
//    string input = "";
//
//    //Get numbers to add together
//    do {
//        cout << "Enter a number, or enter 0 to add all your numbers up: ";
//        cin >> input;
//
//        result += stod(input);
//
//    } while (input != "0");
//
//    cout << "\nThe answer is " << result << ".\n\n";
//}

//bool is_number(const std::string& s) {
//    //This function checks if a string is a number.
//    std::string::const_iterator it = s.begin();
//
//    if (s[0] == '-') {
//        it++;
//    }
//
//    while (it != s.end() && std::isdigit(*it)) ++it;
//    return !s.empty() && it == s.end();
//}

bool is_number_revised(string input) {
    //A revised function that handles decimal point numbers unlike the previous function.
    int index = 0;
    bool decimal_num = false;
    std::string str = input;

//    if (input[0] == '-') {
//        cout << "This is a negative number.\n";
//    }

    for (char& x : str) {
//        cout << x << ".\n";
        if ((x < '0' || x > '9') && x >= '!') {
//            cout << "x is not a number, check if it is a decimal point or negative sign. (If it is a number, we can simply continue to the next iteration.)\n";

            if (x == '.') {
                if (!decimal_num) {
                    decimal_num = true;
                } else {
                    return false;
                }
            } else if (x != '-' || index != 0) { //This condition is checking if it's looking at a negative sign at the beginning of a number. If it is, we can continue, but otherwise we need to return false.
                return false;
            }
        }
        index++; //This is to signify we are at the next index in the string.
    }
    return true;
}

bool is_operator(string operat) {
    //This is a simple operation that checks if a string is a valid operator.
    return (operat == "+" || operat == "-" || operat == "*" || operat == "/" || operat == "//" || operat == "%" || operat == "%%" || operat == "^" || operat == "**");
}

bool is_trig_function(string operat) {
    //This function checks if a string is a valid trigonometry function.
    return (operat == "sin" || operat == "cos" || operat == "tan" || operat == "asin" || operat == "acos" || operat == "atan");
}

bool is_degrees_or_radians(string input) {
    //This function checks if a string is 'r' or 'd', used for trigonometric functions
    return (input == "r" || input == "d");
}

bool is_log_function(string operat) {
    //This function checks if a string is a valid logarithmic function.
    return (operat == "log" || operat == "ln" || operat == "logx" || operat == "e");
}

//double translate_time_to_double(string input, char delimiter) {
//    //This function checks if a string is in proper time format, and translates into a double of seconds if so.
//    //This function uses a delimiter to seperate hours, minutes, and seconds. For example, with a delimiter of ':', 1:20:3 would be an hour, 20 minutes, and 3 seconds.
//    double hours = 0;
//    double minutes = 0;
//    double seconds = 0;
//    int unit = 3; //Keeps track of the current unit we are iterating. 2 = hours, 1 = minutes, 0 = seconds
//    std::string str = input + delimiter; //Adding a delimiter to the end of the string will make sure the last number (seconds) will be calculated.
//    string input_for_current_unit = "";
//
//
//    for (char& x : str) {
//        cout << x << "\n";
//        if (x == delimiter) { //If we meet the delimiter, we need to convert the input for current unit so far into a double, record it, and move on to the next unit
//            if (unit > 0) {
//                cout << "unit is greater than 0\n";
//                switch(unit) {
//                case 3: { //Add input_for_current_unit to hours
//                    if (is_number_revised(input_for_current_unit)) {
//                        hours = stod(input_for_current_unit);
//                        cout << "Hours: " << hours << "\n";
//                    } else {
//                        return NULL;
//                    }
//                    break;
//                }
//                case 2: { //Add input_for_current_unit to minutes
//                    if (is_number_revised(input_for_current_unit)) {
//                        minutes = stod(input_for_current_unit);
//                        cout << "Minutes: " << minutes << "\n";
//                    } else {
//                        return NULL;
//                    }
//                    break;
//                }
//                case 1: { //Add input_for_current_unit to seconds
//                    if (is_number_revised(input_for_current_unit)) {
//                        seconds = stod(input_for_current_unit);
//                        cout << "Seconds: " << seconds << "\n";
//                    } else {
//                        return NULL;
//                    }
//                    break;
//                }
//                default:
//                    cout << "Oops! Something went wrong...\n";
//                    return NULL;
//                }
//                unit--;
//                cout << "unit is now " << unit << "\n";
//                input_for_current_unit = "";
//            }
//            else {
//                return NULL;
//            }
//        } else if (is_number_revised(string(1, x))) {
//            input_for_current_unit += x;
//        } else {
//            return NULL;
//        }
//    }
//
//    return (hours * 60 * 60) + (minutes * 60) + seconds;
//}
//
//string translate_double_to_time(double time, char delimiter) {
//    string output = "";
//
//    int hours = int(time / 3600);
//    time -= hours * 3600;
//
//    int minutes = int(time / 60);
//    time -= minutes * 60;
//
//    double seconds = time;
//
//    return string(hours) + ":" + string(minutes) + ":" + string(seconds);
//}

double get_double_from_string(string input) {
    //This function will get input and return the input as a double.
    while (!is_number_revised(input) && input !=  "x") { //This while loop will keep the program locked until the user enters a valid input (or exits).
        cout << "That's not a valid input. Please enter a number: ";
        cin >> input;
    }

    if (input == "x") { //Input to exit the program.
        return NULL;
    }

    return stod(input);
}

double get_root(double number, double root) {
    //This function will return the root of a given number.
    while (trunc(root) != root || root < 1) {
        //Handle invalid values for the root.
        cout << "That number is invalid as a root. Please enter an integer that is greater than 0: ";
        cin >> root;
    }

    int int_root = int(root);

    if (int_root == 1) {
        return number;
    }

    double upper_bound = 0;
    double lower_bound = 0;

    if (number > 0) {
        upper_bound = number;

    } else if (number < 0) {
        //If we are trying to find an even-numbered root of a negative number, we need to tell the user it is undefined and end the function.
        if (int_root % 2 == 0) {
            cout << "Cannot find the even-numbered root of a negative number.\n";
            return NULL;
        }

        lower_bound = number;

    } else {
        return 0;
    }

    int num_of_guesses = 0;
    double guess = round(number / int_root); //The initial guess to find the correct result.
//    cout << "Guessing: " << guess << "\n";

//    cout << "Difference: " << abs(pow(guess, root) - number);
    while ((abs(pow(guess, int_root) - number) >= 0.000001) && (num_of_guesses < 1000000)) {
        //Iteratively down the guess until we have the correct answer, or until we have guessed a million times, in which case we give our best guess. The million guess limit keeps the program from freezing.

        if (pow(guess, int_root) < number) {
            lower_bound = guess;
//            cout << "New Lower Bound: " << lower_bound << "\n";
        } else if (pow(guess, int_root) > number) {
            upper_bound = guess;
//            cout << "New Upper Bound: " << upper_bound << "\n";
        } else {
            return guess;
        }

        guess = (upper_bound + lower_bound) / 2; //Update the guess and try again.
//        cout << "Guessing: " << guess << "\n";
        num_of_guesses++;
    }
    return guess;
}

double simple_operation_from_string() { //Perform and return a simple operation using number inputs an operator input.
                                        //After the user as performed an operation, they can keep going with more operations until they exit the function.
    //Show a menu with the options the user has.
    cout << "\n";
    cout << "---List of Operation Inputs---\n";
    cout << "Addition:       '+',           Subtraction: '-',    Multiplication:   '*',   Division:  '/'\n";
    cout << "Exponentiation: '^' or '**',   Root:        '//',   Modulus Division: '%',   Remainder: '%%'\n";
    cout << "\n";
    cout << "\n";

    double result = 0;
    string input = "";
    double num1 = 0;
    double num2 = 0;
    string operat = "";

    cout << "Enter the first number, or enter 'x' to exit: ";
    cin >> input;

    num1 = get_double_from_string(input);
    if (num1 == NULL) {
        return num1;
    }

    while (true) { //This loop keeps going until the user returns the function with an exit key or an error occurs. IMPORTANT TO MAKE SURE RETURN STATEMENTS ARE ACCESSIBLE BY USER!
        input = "";
        num2 = 0;
        operat = "";

        cout << "Enter the operator, or enter 'x' to exit: ";
        cin >> input;

        while (!is_operator(input) && input !=  "x") { //This while loop will keep the program locked until the user enters a valid input or exits.
            cout << "That's not a valid input. Please enter one of the operators listed above: ";
            cin >> input;
        }
        if (input == "x") { //Input to exit the program.
            return result;
        }

        operat = input;

        cout << "Enter the next number, or enter 'x' exit: ";
        cin >> input;

        num2 = get_double_from_string(input);
        if (num2 == NULL) {
            return result;
        }

        if (operat == "+") { //Perform the operation.
            result = num1 + num2;
        }
        else if (operat == "-") {
            result = num1 - num2;
            num1 = result;
        }
        else if (operat == "*") {
            result = num1 * num2;
        }
        else if (operat == "/") {
            result = num1 / num2;
        }
        else if (operat == "//") {
            result = get_root(num1, num2);
        }
        else if (operat == "%") {
            result = int(num1 / num2);
        }
        else if (operat == "%%") {
            result = fmod(num1, num2);
        }
        else if (operat == "^" || operat == "**") {
            result = pow(num1, num2);
        }
        else {
            cout << "Oops! Something went wrong...\n";
            return NULL;
        }
        cout << "Result so far: " << result << "\n";
        num1 = result;
    }
}

double trig_operation_from_string() { //Perform a trigonometric operation with a single number and an operation as input.
    //Show a menu with the options the user has.
    cout << "\n";
    cout << "---List of Operation Inputs---\n";
    cout << "sin: sine of input,\tcos: cosine of input,\t\ttan: tangent of input\n";
    cout << "asin: arcsine of input,\tacos: arccosine of input,\tatan: arctangent of input\n";
    cout << "\n";
    cout << "\n";

    string input = "";
    double num = 0;
    string operat = "";
    string format = "";

    //Get input from the user.
    cout << "Enter the operator, or enter 'x' to exit: ";
    cin >> input;

    while (!is_trig_function(input) && input !=  "x") { //This while loop will keep the program locked until the user enters a valid input (or exits).
        cout << "That's not a valid input. Please try again: ";
        cin >> input;
    }
    if (input == "x") { //Input to exit the program.
        return NULL;
    }

    operat = input;

    cout << "Enter a number, or enter 'x' to exit: ";
    cin >> input;

    num = get_double_from_string(input);
    if (num == NULL) {
        return NULL;
    }

    cout << "Enter 'r' for radians, or 'd' for degrees: ";
    cin >> format;

    while(!is_degrees_or_radians(format) && input !=  "x") { //This while loop will keep the program locked until the user enters a valid input (or exits).)
        cout << "That's not a valid input. Please try again: ";
        cin >> format;
    }
    if (input == "x") { //Input to exit the program.
        return NULL;
    }

    if (format == "d") { //Perform the operation.
        //Return in degrees.
        if (operat == "sin") {
            return sin(num) * 180 / M_PI;
        }
        if (operat == "cos") {
            return cos(num) * 180 / M_PI;
        }
        if (operat == "tan") {
            return tan(num) * 180 / M_PI;
        }
        if (operat == "asin") {
            return asin(num) * 180 / M_PI;
        }
        if (operat == "acos") {
            return acos(num) * 180 / M_PI;
        }
        if (operat == "atan") {
            return atan(num) * 180 / M_PI;
        }
    } else {
        //Return in radians.
        if (operat == "sin") {
            return sin(num);
        }
        if (operat == "cos") {
            return cos(num);
        }
        if (operat == "tan") {
            return tan(num);
        }
        if (operat == "asin") {
            return asin(num);
        }
        if (operat == "acos") {
            return acos(num);
        }
        if (operat == "atan") {
            return atan(num);
        }
    }
    cout << "Oops! Something went wrong...\n";
    return NULL;
}

double log_operation_from_string() { //Perform a logarithmic operation with a single number and an operation as input.
    //Show a menu with the options the user has.
    cout << "\n";
    cout << "---List of Operation Inputs---\n";
    cout << "log: base 10 log,\t\tln: natural log\n";
    cout << "logx: log with custom base,\te: 'e' raised to specified power\n";
    cout << "\n";
    cout << "\n";

    string input = "";
    double num = 0;
    double base = 0;
    string operat = "";

    //Get input from the user.
    cout << "Enter the operator, or enter 'x' to exit: ";
    cin >> input;

    while (!is_log_function(input) && input !=  "x") { //This while loop will keep the program locked until the user enters a valid input (or exits).
        cout << "That's not a valid input. Please try again: ";
        cin >> input;
    }
    if (input == "x") { //Input to exit the program.
        return NULL;
    }

    operat = input;

    //If the user inputed a custom log function, get the base.
    if (operat == "logx") {
        cout << "Enter the log base, or enter 'x' to exit: ";
        cin >> input;

        base = get_double_from_string(input);
        if (base == NULL) {
            return NULL;
        }
    }

    cout << "Enter a number, or enter 'x' to exit: ";
    cin >> input;

    num = get_double_from_string(input);
    if (num == NULL) {
        return NULL;
    }

    if (operat == "log") {
        return log10(num);
    }
    if (operat == "ln") {
        return log(num);
    }
    if (operat == "logx") {
        return log10(num) / log10(base);
    }
    if (operat == "e") {
        return exp(num);
    }

    else {
        cout << "Oops! Something went wrong...\n";
        return NULL;
    }
}

int main() {
//    cout << add(3, 5.78) << "\n";
    int choice = -1; //The value for when the user is deciding what to do

    while (choice != 0) {

        //Show menu
        cout << "-----MAIN MENU-----\n";
        cout << "\n";
        cout << "1: Perform a simple operation. (addition, division, etc.)\n";
        cout << "2: Perform a trigonometric operation. (sine, cosine, etc.)\n";
        cout << "3: Perform a logarithmic operation. (log, ln, etc.)\n";
        cout << "0: Exit the program.\n";
        cout << "\n";

        //Process user choice
        cout << "Enter your choice: ";
        cin >> choice;

        switch(choice) {
            case 0: { //Exit
                cout << "Goodbye!";
                break;
            }

            case 1: //Simple operation
            {
                double result = simple_operation_from_string();
                cout << "\n";
                cout << "\nResult: " << result << "\n";
                cout << "\n";
                cout << "\n";
                break;
            }

            case 2: //Trigonometric operation
            {
                double result = trig_operation_from_string();
                cout << "\n";
                cout << "\nResult: " << result << "\n";
                cout << "\n";
                cout << "\n";
                break;
            }

            case 3: //Logarithmic operation
            {
                double result = log_operation_from_string();
                cout << "\n";
                cout << "\n";
                cout << "\nResult: " << result << "\n";
                cout << "\n";
                cout << "\n";
                break;
            }

//            case 4: //Time operation
//            {
//                string input;
//                cout << "Enter a time: ";
//                cin >> input;
//
//                double result = translate_time_to_double(input, ':');
//                cout << "\n";
//                cout << "\n";
//                cout << "\nResult: " << result << "\n";
//                cout << "\n";
//                cout << "\n";
//                break;
//            }

            default:
            {
                cout << "That was not a valid option. Please try again.\n";
            }
        }
    }

    return 0;
}
