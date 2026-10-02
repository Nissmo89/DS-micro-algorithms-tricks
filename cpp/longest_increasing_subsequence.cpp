#include <bits/stdc++.h>
using namespace std;

int lengthOfLIS(const vector<int>& nums) {
    vector<int> tails;
    for (int x : nums) {
        auto it = lower_bound(tails.begin(), tails.end(), x);
        if (it == tails.end())
            tails.push_back(x);
        else
            *it = x;
    }
    return (int)tails.size();
}

// Example usage:
// int main() {
//     vector<int> a = {10,9,2,5,3,7,101,18};
//     cout << lengthOfLIS(a) << '\n'; // outputs 4
//     return 0;
// }
