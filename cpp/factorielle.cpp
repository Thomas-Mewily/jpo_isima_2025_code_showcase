#include <iostream>
#include <iomanip>

int factorielle_recursive(int n) 
{
    switch (n) 
    {
        case 0: case 1:
            return 1;
        default:
            return n * factorielle_recursive(n - 1);
    }
}


int factorielle_recursive_avec_si(int n) 
{
    if (n == 0 || n == 1)
        return 1;
    return n * factorielle_recursive_avec_si(n - 1);
}

int factorielle_iterative(int n) 
{
    int resultat = 1;
    while (n != 0) 
    {
        resultat *= n;
        n -= 1;
    }
    return resultat;
}

int main() {
    for (int i = 0; i < 10; ++i) 
    {
        std::cout << "factorielle(" 
                  << std::setw(2) << i << ") = "
                  << std::setw(7) << std::right << factorielle_recursive(i) << " (recursif) "
                  << std::setw(7) << std::left  << factorielle_iterative(i) << " (iteratif)"
                  << std::endl;
    }
    return 0;
}

/*
g++ -o cpp/factorielle cpp/factorielle.cpp
./cpp/factorielle
*/