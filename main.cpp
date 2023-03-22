// by ElCapitan 
// atdt-032220230755
#include <filesystem>
#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <bits/stdc++.h>

namespace fs = std::filesystem;

using std::cin;
using std::cout;
using std::endl;
using std::string;

class Path 
{
  public:
  string name;
  string path;
  short page;

  //std::vector<std::vector<string>> context;
  std::vector<string> context;

  Path(short page, string path) {
    this->path = path;
    this->page = page;
    auto const pos = path.find_last_of('/');
    this->name = path.substr(pos + 1);

    getContext(path);
  }

  void displayPath() 
  {
    if (this->page == 0) 
    {
      for (short x = 0; x != this->page; ++x) {
        cout << "\t";
      }
      cout << "/" << this->name << endl;
    } 
    else 
    {
      for (short x = 0; x != this->page; ++x) {
        cout << "\t";
      }

      cout << "\t└ /" << this->name << "/" << endl;
    }

    for (auto it = this->context.begin(); it != this->context.end(); ++it) 
    {
      //std::vector<string> ctx = *it;
      if (it == --this->context.end())
      {
        for (short x = 0; x != this->page + 1; ++x) {
          cout << " ";
        }
        cout << "  └ " << *it << endl;
        continue;
      }
      for (short x = 0; x != this->page + 1; ++x) {
        cout << " ";
      }
      cout << "  ├ " << *it << endl;
    }
  }

  private:
  static bool mycomp(string a, string b){
	  return a<b;
  }

  static void alphabaticallySort(std::vector<string> &a){
    int n=a.size();
    sort(a.begin(),a.end(),mycomp);
  }
  
  static void RemoveWord(string word, string &line) 
  {
    auto n = line.find(word);

    if (n != std::string::npos)
    {
      line.erase(n, word.length());
    }
  }

  void getContext(string path) 
  {
    for (const auto & entry : fs::directory_iterator(path)) 
    {
      string pathPart = entry.path();

      RemoveWord(this->path + "/", pathPart);

      if (entry.is_directory()) {
        //std::vector<string> ctx = { pathPart + "/", "dir" };
        this->context.push_back(pathPart + "/");
        continue;
      }
      //std::vector<string> ctx = {pathPart, "file"};
      this->context.push_back(pathPart);
    }
    alphabaticallySort(this->context);
  }
};

string getInput(string PS1) 
{
  string inp;
  cout << PS1;
  cin >> inp;
  return inp;
}

enum Commands 
{
  ls,
  quit
};

int main() {
  string currentDir = fs::current_path();
  Path current(0, currentDir);
  current.displayPath();
  return 0;
}