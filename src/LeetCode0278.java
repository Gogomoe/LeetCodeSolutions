/* The isBadVersion API is defined in the parent class VersionControl.
      boolean isBadVersion(int version); */

class LeetCode0278 {

    public class VersionControl {
        boolean isBadVersion(int version) {
            return true;
        }
    }

    public class Solution extends VersionControl {
        public int firstBadVersion(int n) {
            int l = 1;
            int r = n;
            while (l < r) {
                int mid = l + (r - l) / 2;
                if (isBadVersion(mid)) {
                    r = mid;
                } else {
                    l = mid + 1;
                }
            }
            return l;
        }
    }
}
