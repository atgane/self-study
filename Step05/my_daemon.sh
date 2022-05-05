# 카운터 초기화
COUNT=0

# 환경 변수가 없으면 설정
if [ -z "SINTERVAL" ]; then
    INTERVAL=3
fi

# 메인 루프
while [ true ];
do
    TM=`date|awk '{print $4}'`
    printf "%s : %s \n" $TM $COUNT
    let COUNT=COUNT+1
    sleep $INTERVAL
done