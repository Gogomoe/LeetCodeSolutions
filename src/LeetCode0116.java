public class LeetCode0116 {
    // Definition for a Node.
    private static class Node {
        public int val;
        public Node left;
        public Node right;
        public Node next;

        public Node() {
        }

        public Node(int _val) {
            val = _val;
        }

        public Node(int _val, Node _left, Node _right, Node _next) {
            val = _val;
            left = _left;
            right = _right;
            next = _next;
        }
    }

    class Solution {
        public Node connect(Node root) {
            process(root);
            return root;
        }

        private void process(Node root) {
            if (root == null) {
                return;
            }
            merge(root.left, root.right);
            process(root.left);
            process(root.right);
        }

        private void merge(Node left, Node right) {
            while (left != null && right != null) {
                left.next = right;
                left = left.right;
                right = right.left;
            }
        }
    }
}
