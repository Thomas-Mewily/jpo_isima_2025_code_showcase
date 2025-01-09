int factorielle_iterative(int n)
{
    int resultat = 1;
    while (n != 0)
    {
        resultat *= n;
        n--;
    }
    return resultat;
}






