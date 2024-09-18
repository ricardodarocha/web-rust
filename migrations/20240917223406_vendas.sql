-- Add migration script here
-- Tabela de Dimensão Cliente
CREATE TABLE Cliente (
    ClienteID INT PRIMARY KEY,
    Nome VARCHAR(100),
    Email VARCHAR(100),
    Telefone VARCHAR(15),
    Endereco VARCHAR(255)
);

-- Tabela de Dimensão Produto
CREATE TABLE Produto (
    ProdutoID INT PRIMARY KEY,
    Nome VARCHAR(100),
    Categoria VARCHAR(50),
    Preco DECIMAL(10, 2)
);

-- Tabela de Dimensão Data
CREATE TABLE Data (
    DataID INT PRIMARY KEY,
    Data DATE,
    Ano INT,
    Mes INT,
    Dia INT,
    Trimestre INT,
    NomeMes VARCHAR(20),
    NomeDiaSemana VARCHAR(20)
);

-- Tabela de Fatos Vendas
CREATE TABLE Vendas (
    VendaID INT PRIMARY KEY,
    ClienteID INT,
    ProdutoID INT,
    DataID INT,
    Quantidade INT,
    Total DECIMAL(10, 2),
    Status VARCHAR(20),
    FOREIGN KEY (ClienteID) REFERENCES Cliente(ClienteID),
    FOREIGN KEY (ProdutoID) REFERENCES Produto(ProdutoID),
    FOREIGN KEY (DataID) REFERENCES Data(DataID)
);
