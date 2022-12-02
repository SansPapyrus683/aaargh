#include <iostream>
#include <vector>
#include <map>
#include <algorithm>

using std::cout;
using std::endl;
using std::vector;

class DominikArray {
    private:
        const int POW = 1000003;
        // const int MOD = 1e9 + 9;
        const int MOD = 1000004099;

        vector<int> arr;
        vector<int> sorted;

        vector<int> parent;
        vector<int> size;
        int bad_num = 0;

        std::map<int, long long> elem_pow;  // raise to the power of the value
        vector<long long> hash;
        vector<long long> req_hash;
        std::map<long long, int> bad_diff;
        long long cloud_pairs = 0;

        int get_top(int n) {
            return parent[n] == n ? n : (parent[n] = get_top(parent[n]));
        }

        inline bool is_unsortable(int n) {
            return hash[n] != req_hash[n];
        }

        void add_if_bad(int n) {
            if (is_unsortable(n)) {
                bad_num++;
                long long diff = req_hash[n] - hash[n];
                bad_diff[diff] += size[n];

                long long pair_amt = bad_diff[-diff] + bad_diff[MOD - diff] + bad_diff[-MOD - diff];
                cloud_pairs += pair_amt * size[n];
            }
        }

        void remove_if_bad(int n) {
            if (is_unsortable(n)) {
                bad_num--;
                long long diff = req_hash[n] - hash[n];
                bad_diff[diff] -= size[n];

                long long pair_amt = bad_diff[-diff] + bad_diff[MOD - diff] + bad_diff[-MOD - diff];
                cloud_pairs -= pair_amt * size[n];
            }
        }
    public:
        DominikArray(vector<int> arr)
            : arr(arr), parent(arr.size()), size(arr.size(), 1),
              hash(arr.size()), req_hash(arr.size()) {
            sorted = arr;
            std::sort(sorted.begin(), sorted.end());

            long long curr_pow = 1;
            for (int i : sorted) {
                if (!elem_pow.count(i)) {
                    elem_pow[i] = curr_pow;
                    curr_pow = (curr_pow * POW) % MOD;
                }
            }

            for (int i = 0; i < arr.size(); i++) {
                parent[i] = i;

                hash[i] = elem_pow[arr[i]];
                req_hash[i] = elem_pow[sorted[i]];
                add_if_bad(i);
            }
        }

        void swap(int a, int b) {
            int top_a = get_top(a);
            int top_b = get_top(b);

            remove_if_bad(top_a);
            remove_if_bad(top_b);

            hash[top_a] = hash[top_a] + elem_pow[arr[b]] - elem_pow[arr[a]] + MOD;
            hash[top_a] = hash[top_a] % MOD;
            hash[top_b] = hash[top_b] + elem_pow[arr[a]] - elem_pow[arr[b]] + MOD;
            hash[top_b] = hash[top_b] % MOD;

            add_if_bad(top_a);
            add_if_bad(top_b);

            std::swap(arr[a], arr[b]);
        }

        void link(int a, int b) {
            a = get_top(a);
            b = get_top(b);
            if (a == b) {
                return;
            }

            if (size[a] < size[b]) {
                return link(b, a);
            }

            remove_if_bad(a);
            remove_if_bad(b);

            size[a] += size[b];
            parent[b] = a;

            hash[a] = (hash[a] + hash[b]) % MOD;
            req_hash[a] = (req_hash[a] + req_hash[b]) % MOD;

            add_if_bad(a);
        }

        bool sortable() {
            return bad_num == 0;
        }

        long long needed_pair_num() {
            return cloud_pairs;
        }
};

// https://oj.uz/problem/view/COCI16_zamjene (input omitted due to length)
int main() {
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);

    int arr_len;
    int query_num;
    std::cin >> arr_len >> query_num;
    vector<int> arr(arr_len);
    for (int& i : arr) {
        std::cin >> i;
    }

    DominikArray array(arr);
    for (int q = 0; q < query_num; q++) {
        int type;
        std::cin >> type;
        int a, b;  // not necessarily used (queries of type 3 & 4)
        switch (type) {
            case 1:
                std::cin >> a >> b;
                array.swap(--a, --b);
                break;
            case 2:
                std::cin >> a >> b;
                array.link(--a, --b);
                break;
            case 3:
                cout << (array.sortable() ? "DA" : "NE") << '\n';
                break;
            case 4:
                cout << array.needed_pair_num() << '\n';
                break;
        };
    }
}
