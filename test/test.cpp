#include <iostream>
#include <fstream>
#include <vector>

using namespace std;

int main() {
    cout << "2" << endl;
    vector<pair<int, int>> vec{{1, 2}, {3, 4}};
    for (const auto& [a, b] : vec) {
        int x = a + b;
        cout << x << endl;
    }
}
