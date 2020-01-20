import java.util.*;
import java.util.stream.Collectors;

public class LeetCode0690 {

    // Employee info
    class Employee {
        // It's the unique id of each node;
        // unique id of this employee
        public int id;
        // the importance value of this employee
        public int importance;
        // the id of direct subordinates
        public List<Integer> subordinates;
    }

    class Solution {
        public int getImportance(List<Employee> employees, int id) {
            Map<Integer, Employee> employeeMap = employees.stream()
                    .collect(Collectors.toMap(it -> it.id, it -> it));
            int result = 0;
            Queue<Employee> queue = new ArrayDeque<>();

            Employee root = employeeMap.get(id);
            if (root != null) {
                queue.add(root);
            }

            while (!queue.isEmpty()) {
                Employee it = queue.poll();
                result += it.importance;
                for (Integer subordinate : it.subordinates) {
                    Employee sub = employeeMap.get(subordinate);
                    if (sub != null) {
                        queue.add(sub);
                    }
                }
            }

            return result;

        }
    }
}
