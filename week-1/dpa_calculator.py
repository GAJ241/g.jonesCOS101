# ----------------------------------------
# Departmental Performance Average (DPA) Calculator
# ----------------------------------------

def calculate_dpa():
    print("=== Departmental Performance Average (DPA) Calculator ===")
    
    total_points = 0
    total_units = 0
    
    # Ask how many courses
    num_courses = int(input("Enter number of courses: "))

    # Loop through each course
    for i in range(num_courses):
        print(f"\nCourse {i+1}")
        course_name = input("Course name: ")
        grade_point = float(input(f"Grade point for {course_name} (A=5, B=4, C=3, D=2, E=1, F=0): "))
        course_units = float(input(f"Units for {course_name}: "))

        total_points += grade_point * course_units
        total_units += course_units

    # Calculate DPA
    dpa = total_points / total_units
    print("\n----------------------------------------")
    print(f"Your Departmental Performance Average (DPA) is: {round(dpa, 2)}")
    print("----------------------------------------")


# Run the calculator
calculate_dpa()
