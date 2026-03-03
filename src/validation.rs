impl Arguments {
    pub async fn try_check_unique_constraint(&self, _conn: &PgPool) -> Result<&Self, ClientError> {
        todo!()
    }
    pub async fn try_check_if_entry_exists(
        &self,
        _conn: impl PgExecutor<'_>,
    ) -> Result<&Self, ClientError> {
        let mut q = ("SELECT");
        todo!()
    }
    async fn select(&self, conn: impl PgExecutor<'_>) -> Result<Vec<String>, sqlx::Error> {
        let mut q = QueryBuilder::<Postgres>::new("SELECT ");
        q.push(&self.column);
        q.push(" FROM ");
        q.push(&self.table);
        q.push(" WHERE ");
        q.push(&self.column);
        q.push(" IN");
        q.push_tuples(self.args.iter().take(self.bind_limit), |mut b, col| {
            if self.uuid {
                b.push_bind(Uuid::parse_str(col).expect("UUID's should already been validated."));
            } else {
                b.push_bind(col);
            }
        });

        if self.uuid {
            let res = q.build_query_scalar::<Uuid>().fetch_all(conn).await?;
            Ok(res.into_iter().map(|f| f.to_string()).collect())
        } else {
            q.build_query_scalar::<String>().fetch_all(conn).await
        }
    }
    /// Test case where arguments are expected to exist
    pub fn entry_not_found() -> (Arc<[String]>, String) {
        (
            Arc::new(["balto".into(), "air bud".into()]),
            "balto, air bud".into(),
        )
    }
}
