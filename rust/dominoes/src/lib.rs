pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let mut edges = EdgesMatrix::new(input)?;

    edges
        .degree_satisfied()
        .then(|| edges.make_circuit())
        .flatten()
        .map(|vertices| {
            vertices
                .windows(2)
                .map(|domino| (domino[0], domino[1]))
                .collect()
        })
}

fn domino_position(source_half: u8, sink_half: u8) -> usize {
    ((source_half - 1) * DOMINO_LIMIT + (sink_half - 1)) as usize
}

const DOMINO_LIMIT: u8 = 6;
const MATRIX_SIZE: usize = DOMINO_LIMIT.pow(2) as usize;

struct EdgesMatrix {
    matrix: [u8; MATRIX_SIZE],
}

impl EdgesMatrix {
    fn new(input: &[(u8, u8)]) -> Option<Self> {
        let mut matrix = [0; MATRIX_SIZE];

        for domino in input {
            match domino {
                &(left_half @ 1..=DOMINO_LIMIT, right_half @ 1..=DOMINO_LIMIT) => {
                    matrix[domino_position(left_half, right_half)] += 1;
                    matrix[domino_position(right_half, left_half)] += 1;
                }
                _ => return None,
            }
        }

        Some(Self { matrix })
    }

    fn degree_satisfied(&self) -> bool {
        self.matrix
            .chunks(DOMINO_LIMIT as usize)
            .all(|edges| edges.iter().sum::<u8>() % 2 == 0)
    }

    fn make_circuit(&mut self) -> Option<Vec<u8>> {
        let mut stack = Vec::new();
        let mut circuit = Vec::new();

        if let Some(pos) = self.matrix.iter().position(|&num_edges| num_edges > 0) {
            let mut current_half = pos as u8 / DOMINO_LIMIT + 1;

            loop {
                match self.pick_next_vertex(current_half) {
                    Some(next_half) => {
                        stack.push(current_half);
                        current_half = next_half;
                    }
                    _ => {
                        circuit.push(current_half);
                        match stack.pop() {
                            Some(next_half) => current_half = next_half,
                            _ => break,
                        }
                    }
                }
            }
        }

        self.matrix
            .iter()
            .all(|&value| value == 0)
            .then_some(circuit)
    }

    fn pick_next_vertex(&mut self, domino_half: u8) -> Option<u8> {
        let row = {
            let lower_bound = ((domino_half - 1) * DOMINO_LIMIT) as usize;
            let upper_bound = (domino_half * DOMINO_LIMIT) as usize;
            &self.matrix[lower_bound..upper_bound]
        };

        let next_half = row.iter().position(|&value| value != 0)? as u8 + 1;

        self.matrix[domino_position(domino_half, next_half)] -= 1;
        self.matrix[domino_position(next_half, domino_half)] -= 1;

        Some(next_half)
    }
}
