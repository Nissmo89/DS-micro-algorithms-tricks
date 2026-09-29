#include <bits/stdc++.h>
using namespace std;

int maxSumRectangle(const vector<vector<int>>& matrix) {
    int rows = matrix.size();
    if(rows==0) return 0;
    int cols = matrix[0].size();
    int maxSum = INT_MIN;
    for(int left=0; left<cols; ++left){
        vector<int> temp(rows,0);
        for(int right=left; right<cols; ++right){
            for(int i=0;i<rows;++i){
                temp[i] += matrix[i][right];
            }
            int current = temp[0];
            int best = temp[0];
            for(int i=1;i<rows;++i){
                current = max(temp[i], current + temp[i]);
                best = max(best, current);
            }
            maxSum = max(maxSum, best);
        }
    }
    return maxSum;
}

int main(){
    vector<vector<int>> mat = {
        {1, 2, -1, -4, -20},
        {-8, -3, 4, 2, 1},
        {3, 8, 10, 1, 3},
        {-4, -1, 1, 7, -6}
    };
    cout << maxSumRectangle(mat) << endl;
    return 0;
}