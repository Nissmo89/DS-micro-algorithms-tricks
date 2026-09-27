public class KadaneWithIndices {
    static class Result {
        int maxSum;
        int start;
        int end;
        Result(int maxSum, int start, int end) {
            this.maxSum = maxSum;
            this.start = start;
            this.end = end;
        }
    }

    public static Result maxSubarray(int[] arr) {
        int maxSoFar = arr[0];
        int maxEndingHere = arr[0];
        int start = 0;
        int tempStart = 0;
        int end = 0;
        for (int i = 1; i < arr.length; i++) {
            if (arr[i] > maxEndingHere + arr[i]) {
                maxEndingHere = arr[i];
                tempStart = i;
            } else {
                maxEndingHere += arr[i];
            }
            if (maxEndingHere > maxSoFar) {
                maxSoFar = maxEndingHere;
                start = tempStart;
                end = i;
            }
        }
        return new Result(maxSoFar, start, end);
    }

    public static void main(String[] args) {
        int[] arr = {-2, -3, 4, -1, -2, 1, 5, -3};
        Result r = maxSubarray(arr);
        System.out.println("Max Sum: " + r.maxSum);
        System.out.println("Start Index: " + r.start + ", End Index: " + r.end);
    }
}
