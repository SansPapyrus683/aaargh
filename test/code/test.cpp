#include <iostream>
#include <vector>
#include <algorithm>
#include <numeric>

using namespace std;

int main() {
    int size;
    cin >> size;
    vector<int> arr(size);
    for (int& i : arr) {
        cin >> i;
    }

    cout << accumulate(arr.begin(), arr.end(), 0) << '\n';
    cout << *min_element(arr.begin(), arr.end()) << endl;
}
