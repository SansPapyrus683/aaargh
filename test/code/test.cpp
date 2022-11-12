#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>

using namespace std;

int main() {
    int size;
    cin >> size;

    int total = 0;
    int min = INT32_MAX;
    for (int i = 0; i < size; i++) {
        int x;
        cin >> x;
        total += x;
        min = std::min(min, x);
    }

    cout << total + 1 << '\n';
    cout << min << endl;
}
