use super::Natural;

use std::cmp;
use std::ops::Sub;

impl Sub for &Natural {
    type Output = Natural;

    fn sub(self, rhs: Self) -> Natural {
        let mut sum = Natural::new(needed_capacity(self, rhs));

        let mut lhs = self.clone(); // TODO: 左辺をclone()しなくても繰り下がり処理をできるようにする
        for i in 0..sum.length {
            if lhs[i] >= rhs[i] {
                sum[i] = lhs[i] - rhs[i];
            } else {
                sum[i] = (usize::MAX - (rhs[i] - lhs[i])) + 1; // 繰り下げて計算
                for j in (i + 1)..sum.length {
                    if lhs[j] > 0 {
                        lhs[j] -= 1;
                        break;
                    }
                    lhs[j] = usize::MAX;
                }
            }
        }
        sum.fit();
        sum
    }
}

fn needed_capacity(lhs: &Natural, rhs: &Natural) -> usize {
    cmp::max(lhs.length, rhs.length) // TODO: もうちょっと賢くする
}
