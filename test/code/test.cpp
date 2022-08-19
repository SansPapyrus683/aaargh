#include <iostream>
#include <fstream>
#include <vector>

using namespace std;

int main() {
    int size;
    cin >> size;
    int total = 0;
    for (int i = 0; i < size; i++) {
        int x;
        cin >> x;
        total += x;
    }
    cout << total << endl;
}
