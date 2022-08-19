import java.io.*;
import java.util.*;

/**
 * 2021 dec silver
 * 2 5
 * 1 3
 * 2 5
 * should output these numbers, each on a new line:
 * [0, 0, 1, 3, 4, 4, 4, 3, 3, 1, 1]
 */
public final class ConvIntervals {
    public static void main(String[] args) throws IOException {
        long start = System.currentTimeMillis();
        BufferedReader read = new BufferedReader(new InputStreamReader(System.in));
        StringTokenizer initial = new StringTokenizer(read.readLine());
        int intervalNum = Integer.parseInt(initial.nextToken());
        int maxMag = Integer.parseInt(initial.nextToken());
        int[] startNum = new int[maxMag + 1];
        int[] endNum = new int[maxMag + 1];
        for (int i = 0; i < intervalNum; i++) {
            int[] interval = Arrays.stream(read.readLine().split(" ")).mapToInt(Integer::parseInt).toArray();
            if (interval[0] > interval[1]) {
                throw new IllegalArgumentException("invalid interval i hate you");
            }
            startNum[interval[0]]++;
            endNum[interval[1]]++;
        }

        long[] coveredNum = new long[2 * maxMag + 2];
        for (int i = 0; i <= maxMag; i++) {
            for (int j = 0; j <= maxMag; j++) {
                coveredNum[i + j] += (long) startNum[i] * startNum[j];
                coveredNum[i + j + 1] -= (long) endNum[i] * endNum[j];
            }
        }

        // get the actual array from the initial stuff we constructed
        long currAmt = 0;
        for (int i = 0; i <= 2 * maxMag; i++) {
            currAmt += coveredNum[i];
            coveredNum[i] = currAmt;
        }

        StringBuilder ans = new StringBuilder();
        for (int i = 0; i <= 2 * maxMag; i++) {
            ans.append(coveredNum[i]).append('\n');
        }
        System.out.print(ans);
        System.err.printf("it took %d ms, i hope you're happy", System.currentTimeMillis() - start);
    }
}
