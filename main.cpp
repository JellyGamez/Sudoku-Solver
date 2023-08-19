#include <fstream>
#include <iostream>
#include <chrono>
using namespace std;
fstream f("./datasets/1000000.txt", ios::in);
int main()
{
    int board[10][10];
    for (int i = 0; i < 10; i++)
        for (int j = 0; j < 10; j++)
            board[i][j] = 0;
    auto start = chrono::steady_clock::now();
    string line;
    while (f >> line)
    {
        //for (int i = 0; i < line.size(); i++)
        // {
        //     int row = i / 9 + 1, col = i % 9 + 1;
        //     board[row][col] = line[i] - '0';
        // }
    }
    auto stop = chrono::steady_clock::now();
    cout << chrono::duration_cast<chrono::milliseconds>(stop - start).count();
}