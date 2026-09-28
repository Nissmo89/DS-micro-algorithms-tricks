public class FenwickTree {
    private final int[] tree;
    private final int n;

    public FenwickTree(int size) {
        this.n = size;
        this.tree = new int[n + 1];
    }

    public void update(int index, int delta) {
        for (int i = index + 1; i <= n; i += i & -i) {
            tree[i] += delta;
        }
    }

    public int query(int index) {
        int sum = 0;
        for (int i = index + 1; i > 0; i -= i & -i) {
            sum += tree[i];
        }
        return sum;
    }

    public int rangeSum(int left, int right) {
        return query(right) - (left > 0 ? query(left - 1) : 0);
    }

    public static void main(String[] args) {
        FenwickTree ft = new FenwickTree(5);
        ft.update(0, 1);
        ft.update(1, 2);
        ft.update(2, 3);
        ft.update(3, 4);
        ft.update(4, 5);
        System.out.println("Sum[0..4] = " + ft.rangeSum(0, 4));
        ft.update(2, -1);
        System.out.println("Sum[1..3] = " + ft.rangeSum(1, 3));
    }
}