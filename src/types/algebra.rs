trait Group<T> where T: Field {
    fn mul_id(&self) -> Self;
    fn mul(&self, Self) -> Self;
    fn mul_inv(&self, Self) -> Self;
}

trait Field {
    fn mul_id(&self) -> Self;
    fn mul(&self, Self) -> Self;
    fn mul_inv(&self, Self) -> Self;

    fn add_id(&self) -> Self;
    fn add(&self, Self) -> Self;
    fn add_inv(&self, Self) -> Self;
}
