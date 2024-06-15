-- DDL Statements
CREATE TABLE employees (
    id INT PRIMARY KEY,
    first_name VARCHAR(50),
    last_name VARCHAR(50),
    birth_date DATE,
    hire_date DATE,
    department_id INT
);

CREATE TABLE departments (
    id INT PRIMARY KEY,
    name VARCHAR(100),
    location VARCHAR(100)
);

CREATE TABLE salaries (
    employee_id INT,
    amount DECIMAL(10, 2),
    pay_date DATE,
    PRIMARY KEY (employee_id, pay_date)
);

-- DML Statements
INSERT INTO employees (id, first_name, last_name, birth_date, hire_date, department_id)
VALUES (1, 'John', 'Doe', '1980-01-01', '2005-05-01', 1),
       (2, 'Jane', 'Smith', '1985-02-02', '2010-06-15', 2),
       (3, 'Emily', 'Jones', '1990-03-03', '2015-07-20', 3);

INSERT INTO departments (id, name, location)
VALUES (1, 'HR', 'New York'),
       (2, 'Engineering', 'San Francisco'),
       (3, 'Marketing', 'Chicago');

INSERT INTO salaries (employee_id, amount, pay_date)
VALUES (1, 60000.00, '2023-01-01'),
       (2, 80000.00, '2023-01-01'),
       (3, 75000.00, '2023-01-01');

-- Basic Select
SELECT * FROM employees;

-- Select with Join
SELECT e.first_name, e.last_name, d.name AS department, s.amount AS salary
FROM employees e
JOIN departments d ON e.department_id = d.id
JOIN salaries s ON e.id = s.employee_id
WHERE s.pay_date = '2023-01-01';

-- Select with Subquery
SELECT first_name, last_name
FROM employees
WHERE department_id = (SELECT id FROM departments WHERE name = 'HR');

-- Aggregate Functions
SELECT department_id, AVG(amount) AS avg_salary
FROM salaries
GROUP BY department_id;

-- Complex Query with Subquery and Aggregation
SELECT d.name AS department, COUNT(e.id) AS employee_count, AVG(s.amount) AS avg_salary
FROM departments d
JOIN employees e ON d.id = e.department_id
JOIN salaries s ON e.id = s.employee_id
WHERE s.pay_date = '2023-01-01'
GROUP BY d.name;

-- Update Statement
UPDATE employees
SET department_id = 2
WHERE id = 1;

-- Delete Statement
DELETE FROM salaries
WHERE pay_date < '2023-01-01';

-- Transactions
BEGIN;
UPDATE employees
SET hire_date = '2015-01-01'
WHERE id = 2;

INSERT INTO salaries (employee_id, amount, pay_date)
VALUES (2, 85000.00, '2023-06-01');

COMMIT;

-- Error Handling
BEGIN;
UPDATE employees
SET first_name = 'John'
WHERE id = 100; -- This will fail if no such employee exists

ROLLBACK;
