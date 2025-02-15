def factorielle_recursive(n):
    match n:
        case 0 | 1: return 1 
        case _: return n * factorielle_recursive(n-1)
    

def factorielle_recursive_avec_si(n):
    if (n == 0 or n == 1):
        return 1
    return n * factorielle_recursive_avec_si(n-1)


def factorielle_iterative(n):
    resultat = 1
    while n != 0:
        resultat *= n
        n -= 1
    return resultat


for i in range(10):
    print(f"factorielle({i: >2}) = {factorielle_recursive(i): >7} (recursif) {factorielle_iterative(i): <7} (iteratif)")
print()