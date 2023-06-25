## Ownership & Borrowing | Propriedade e Empréstimo

Este código demonstra os conceitos de propriedade (ownership) e empréstimo (borrowing) na linguagem de programação Rust.
Atribuição por cópia (Copy Attribution)

```
let a = 1;
let b = a;
```

Nesse trecho, o valor 1 é atribuído à variável a. Em seguida, a variável b recebe uma cópia do valor contido em a. Isso ocorre porque o tipo i32 (um tipo de dado primitivo) possui a trait Copy implementada, permitindo que valores sejam copiados diretamente.
Atribuição por referência (Reference Attribution)

```
let a = 1;
let b = &a;
```

Neste caso, a variável a recebe o valor 1. A variável b é então definida como uma referência a a, ou seja, ela aponta para o valor contido em a na memória. Ambas as variáveis compartilham a mesma alocação de memória.

### Posse (Ownership)

```
let a2 = String::from("Rust");
let b2 = &a2;
```

Nesse exemplo, a variável a2 recebe a propriedade de uma String alocada no heap da memória. No entanto, ao tentar atribuir a2 a b2, ocorrerá um erro de compilação. Isso acontece porque tipos como String possuem a trait Drop implementada, o que significa que eles possuem uma semântica de propriedade exclusiva em Rust. Portanto, a propriedade da String não pode ser copiada para b2 sem invalidar a propriedade anterior.

### Empréstimo (Borrowing)

```
let b2 = &a2;
```

Para resolver o problema de posse, é possível fazer um empréstimo (borrow) de a2 através da criação de uma referência b2. A referência b2 permite acessar o valor da String sem assumir a propriedade, ou seja, sem invalidar a2. Isso é conhecido como empréstimo imutável, pois b2 não pode alterar o valor da String enquanto estiver emprestado.

### Mutabilidade e alteração do valor emprestado

```
let mut name = "Username".to_string();
to_uppercase(&mut name);
println!("{name}");
```

Neste trecho, a variável name é definida como uma String mutável contendo o valor "Username". Em seguida, é chamada a função to_uppercase, que recebe um empréstimo mutável (&mut) da String name. Dentro da função, o valor emprestado é convertido para letras maiúsculas e alterado. Ao retornar à função main, o valor de name foi alterado e, portanto, é impresso em letras maiúsculas no println!.