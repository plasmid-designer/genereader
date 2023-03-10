use pest::{iterators::Pair, RuleType};

pub(crate) trait RuleExt
where
    Self: RuleType + PartialEq,
{
    type ERROR;

    fn is(pair: Pair<Self>, expected: Self) -> Option<Pair<Self>> {
        pair.as_rule().eq(&expected).then_some(pair)
    }

    fn expect(pair: Pair<Self>, expected: Self) -> Result<Pair<Self>, Self::ERROR> {
        let actual = pair.as_rule();
        actual
            .eq(&expected)
            .then_some(pair)
            .ok_or_else(|| Self::to_error(Some(expected), Some(actual)))
    }

    fn expect_some(pair: Option<Pair<Self>>, expected: Self) -> Result<Pair<Self>, Self::ERROR> {
        let Some(pair) = pair else {
            return Err(Self::to_error(Some(expected), None));
        };
        Self::expect(pair, expected)
    }

    fn expect_none(pair: Option<Pair<Self>>) -> Result<(), Self::ERROR> {
        if let Some(pair) = pair {
            Err(Self::to_error(None, Some(pair.as_rule())))
        } else {
            Ok(())
        }
    }

    fn to_error(expected: Option<Self>, actual: Option<Self>) -> Self::ERROR;
}

pub(crate) trait PairExt<'a, R>
where
    R: RuleExt,
{
    fn is(self, expected: R) -> Option<Pair<'a, R>>;
    fn expect(self, expected: R) -> Result<Pair<'a, R>, R::ERROR>;
}

pub(crate) trait PairOptionExt<'a, R>
where
    R: RuleExt,
{
    fn expect_some(self, expected: R) -> Result<Pair<'a, R>, R::ERROR>;
    fn expect_none(self) -> Result<(), R::ERROR>;
}

impl<'a, R> PairExt<'a, R> for Pair<'a, R>
where
    R: RuleExt,
{
    fn is(self, expected: R) -> Option<Pair<'a, R>> {
        R::is(self, expected)
    }

    fn expect(self, expected: R) -> Result<Pair<'a, R>, R::ERROR> {
        R::expect(self, expected)
    }
}

impl<'a, R> PairOptionExt<'a, R> for Option<Pair<'a, R>>
where
    R: RuleExt,
{
    fn expect_some(self, expected: R) -> Result<Pair<'a, R>, <R as RuleExt>::ERROR> {
        R::expect_some(self, expected)
    }

    fn expect_none(self) -> Result<(), <R as RuleExt>::ERROR> {
        R::expect_none(self)
    }
}
