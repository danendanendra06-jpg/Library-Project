
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// 1. Struktur data Buku
#[contracttype]
#[derive(Clone, Debug)]
pub struct Book {
    id: u64,
    title: String,
    author: String,
    is_borrowed: bool,
}

// Storage key
const BOOK_DATA: Symbol = symbol_short!("BK_DATA");

#[contract]
pub struct LibraryContract;

#[contractimpl]
impl LibraryContract {

    // Menambah buku baru ke perpustakaan
    pub fn add_book(env: Env, title: String, author: String) -> u64 {
        let mut books: Vec<Book> = env.storage().instance().get(&BOOK_DATA).unwrap_or(Vec::new(&env));
        
        let book_id = env.prng().gen::<u64>();
        let new_book = Book {
            id: book_id,
            title,
            author,
            is_borrowed: false,
        };

        books.push_back(new_book);
        env.storage().instance().set(&BOOK_DATA, &books);
        
        book_id
    }

    // Mengambil semua daftar buku
    pub fn list_books(env: Env) -> Vec<Book> {
        env.storage().instance().get(&BOOK_DATA).unwrap_or(Vec::new(&env))
    }

    // Meminjam buku berdasarkan ID
    pub fn borrow_book(env: Env, id: u64) -> String {
        let mut books: Vec<Book> = env.storage().instance().get(&BOOK_DATA).unwrap_or(Vec::new(&env));
        let mut found = false;

        for i in 0..books.len() {
            let mut book = books.get(i).unwrap();
            if book.id == id {
                if book.is_borrowed {
                    return String::from_str(&env, "Buku sudah dipinjam");
                }
                
                // Update status pinjam
                book.is_borrowed = true;
                books.set(i, book);
                found = true;
                break;
            }
        }

        if found {
            env.storage().instance().set(&BOOK_DATA, &books);
            String::from_str(&env, "Berhasil meminjam buku")
        } else {
            String::from_str(&env, "Buku tidak ditemukan")
        }
    }

    // Mengembalikan buku
    pub fn return_book(env: Env, id: u64) -> String {
        let mut books: Vec<Book> = env.storage().instance().get(&BOOK_DATA).unwrap_or(Vec::new(&env));
        
        for i in 0..books.len() {
            let mut book = books.get(i).unwrap();
            if book.id == id {
                book.is_borrowed = false;
                books.set(i, book);
                env.storage().instance().set(&BOOK_DATA, &books);
                return String::from_str(&env, "Buku telah dikembalikan");
            }
        }
        String::from_str(&env, "ID Buku salah")
    }
}