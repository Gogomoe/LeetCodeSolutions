SELECT d.Name AS Department, e.Name AS Employee, e.Salary AS Salary
FROM Employee AS e
JOIN Department AS d
ON e.DepartmentId = d.Id
WHERE (
    SELECT COUNT(DISTINCT Salary)
    FROM Employee AS e2
    WHERE e2.DepartmentId = e.DepartmentId AND e2.Salary >= e.Salary
    ) <= 3;