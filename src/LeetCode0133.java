import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class LeetCode0133 {
    class Node {
        public int val;
        public List<Node> neighbors;

        public Node() {
            val = 0;
            neighbors = new ArrayList<Node>();
        }

        public Node(int _val) {
            val = _val;
            neighbors = new ArrayList<Node>();
        }

        public Node(int _val, ArrayList<Node> _neighbors) {
            val = _val;
            neighbors = _neighbors;
        }
    }

    class Solution {
        public Node cloneGraph(Node node) {
            if (node == null) return null;
            Map<Node, Node> map = new HashMap<>();
            helper(node, map);
            return map.get(node);
        }

        private Node helper(Node node, Map<Node, Node> map) {
            if (map.containsKey(node)) {
                return map.get(node);
            }
            Node clone = new Node(node.val);
            map.put(node, clone);
            for (Node neighbor : node.neighbors) {
                clone.neighbors.add(helper(neighbor, map));
            }
            return clone;
        }

    }
}
