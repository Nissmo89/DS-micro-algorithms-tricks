#include <bits/stdc++.h>
using namespace std;

string longestPalindromicSubstring(const string &s) {
    int n = s.size();
    string t = "#";
    for(char c: s) t += string(1,c) + string(1,'#');
    int m = t.size();
    vector<int> p(m,0);
    int c=0,r=0;
    for(int i=1;i<m-1;i++){
        int mir = 2*c - i;
        if(i<r) p[i] = min(r-i, p[mir]);
        while(i+p[i]+1<m && i-p[i]-1>=0 && t[i+p[i]+1]==t[i-p[i]-1]) p[i]++;
        if(i+p[i]>r){c=i;r=i+p[i];}
    }
    int maxLen=0,start=0;
    for(int i=1;i<m-1;i++){
        if(p[i]>maxLen){maxLen=p[i];start=(i-maxLen)/2;}
    }
    return s.substr(start,maxLen);
}

int main(){
    string s="babad";
    cout<<"Longest palindrome: "<<longestPalindromicSubstring(s)<<"\n";
    return 0;
}
