import java.io.*;
import java.util.*;

// 2020 us open gold
public final class Haircut {
    public static void main(String[] args) throws IOException {
        long start = System.currentTimeMillis();
        BufferedReader read = new BufferedReader(new FileReader("haircut.in"));
        int hairNum = Integer.parseInt(read.readLine());
        int maxLen = hairNum - 1;  // i mean we only query up to here, so might as well
        int[] hairs = Arrays.stream(read.readLine().split(" ")).mapToInt(h -> Math.min(Integer.parseInt(h), maxLen)).toArray();

        BITree strandCount = new BITree(maxLen + 1);
        long[] inversionContributions = new long[maxLen + 1];
        long total = 0;
        for (int h : hairs) {
            // get all the strands that are greater than this strand (long bc i'm not taking any chances)
            long thisContrib = strandCount.query(maxLen) - strandCount.query(h);
            inversionContributions[h] += thisContrib;
            total += thisContrib;
            strandCount.increment(h, 1);  // log this into the thing
        }

        long[] badnessLevels = new long[maxLen + 1];
        for (int h = maxLen; h >= 0; h--) {
            // when we cut it down to that, all the inversions will disappear (like a miracle)
            total -= inversionContributions[h];
            badnessLevels[h] = total;
        }

        PrintWriter written = new PrintWriter("haircut.out");
        for (long b : badnessLevels) {
            written.println(b);
            System.out.println(b);
        }
        written.close();
        System.out.printf("bam %d ms now leave me alone%n", System.currentTimeMillis() - start);
    }
}

class BITree {
    private final long[] treeThing;
    private final int size;

    public BITree(int size) {
        treeThing = new long[size + 1];
        this.size = size;
    }

    public void increment(int updateAt, long val) {
        updateAt++;  // have the driver code not worry about 1-indexing
        for (; updateAt <= size; updateAt += updateAt & -updateAt) {
            treeThing[updateAt] += val;
        }
    }

    public long query(int ind) {  // the bound is inclusive i think
        ind++;
        long sum = 0;
        for (; ind > 0; ind -= ind & -ind) {
            sum += treeThing[ind];
        }
        return sum;
    }
}
