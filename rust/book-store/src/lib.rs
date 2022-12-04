pub fn lowest_price(books: &[u32]) -> u32 {
    let mut best_price = None;
    let mut books_quantity = BookCounter::default();

    books.iter().for_each(|&book| {
        let book_pos = book as usize - 1;
        if (0..BOOKS_AVAILABLE).contains(&book_pos) {
            books_quantity[book_pos] += 1;
        }
    });

    let num_different_books = books_quantity.iter().filter(|&&amount| amount > 0).count();

    for pack_size in (0..num_different_books).rev() {
        let candidate_price = Some(pack_price(books_quantity, pack_size));
        if best_price.is_none() || candidate_price < best_price {
            best_price = candidate_price;
        } else {
            break;
        }
    }

    best_price.unwrap_or_default()
}

const BOOKS_AVAILABLE: usize = 5;
const BASE_PRICE: f64 = 800.0;
const PACK_DISCOUNT: [f64; BOOKS_AVAILABLE] = [0.0, 0.05, 0.10, 0.20, 0.25];

type BookCounter = [u32; BOOKS_AVAILABLE];

fn pack_price(mut books_quantity: BookCounter, pack_size: usize) -> u32 {
    let mut price = 0;

    loop {
        books_quantity.sort_unstable();
        let number_packs = books_quantity[BOOKS_AVAILABLE - pack_size - 1];
        if number_packs > 0 {
            price += calculate_price(number_packs, pack_size);
            remove(&mut books_quantity, number_packs, pack_size);
        } else {
            if pack_size != 0 {
                price += pack_price(books_quantity, pack_size - 1);
            }
            break price;
        }
    }
}

fn remove(books_quantity: &mut BookCounter, number_packs: u32, pack_size: usize) {
    books_quantity
        .iter_mut()
        .rev()
        .take(pack_size + 1)
        .for_each(|quantity| {
            *quantity -= number_packs;
        });
}

fn calculate_price(number_packs: u32, pack_size: usize) -> u32 {
    (pack_size + 1) as u32 * number_packs * (BASE_PRICE * (1.0 - PACK_DISCOUNT[pack_size])) as u32
}
