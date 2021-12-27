Imports System

Module Program
    Sub Main()
        ' Inputs go here
        Dim p1 = 7
        Dim p2 = 10

        Console.WriteLine($"Task 1: {Task1(p1, p2)}")
        Console.WriteLine($"Task 2: {Task2(p1, p2)}")
    End Sub

    Class GameState
        Public ReadOnly P1 As PlayerState
        Public ReadOnly P2 As PlayerState
        Public ReadOnly Player1Turn As Boolean

        Public Sub New(p1 As Long, p2 As Long)
            Me.New(New PlayerState(p1, 0), New PlayerState(p2, 0), True)
        End Sub

        Public Sub New(p1 As PlayerState, p2 As PlayerState, player1Turn As Boolean)
            Me.Player1Turn = player1Turn
            Me.P1 = p1
            Me.P2 = p2
        End Sub

        Public ReadOnly Property CurrentPlayer As PlayerState
            Get
                Return If(Player1Turn, P1, P2)
            End Get
        End Property

        Public ReadOnly Property PreviousPlayer As PlayerState
            Get
                Return If(Player1Turn, P2, P1)
            End Get
        End Property

        Function Updated(a As Long) As GameState
            If Player1Turn Then
                Return New GameState(P1.Updated(a), P2, Not Player1Turn)
            Else
                Return New GameState(P1, P2.Updated(a), Not Player1Turn)
            End If
        End Function

        Public Overrides Function Equals(obj As Object) As Boolean
            If Not TypeOf obj Is GameState Then Return False
            Dim ps = CType(obj, GameState)
            Return P1.Equals(ps.P1) And P2.Equals(ps.P2) And Player1Turn = ps.Player1Turn
        End Function

        Public Overrides Function GetHashCode() As Integer
            Return HashCode.Combine(P1, P2, Player1Turn)
        End Function

        Public Overrides Function ToString() As String
            Return $"P1: {P1}; P2: {P2}; Turn: {If(Player1Turn, 1, 2)}"
        End Function
    End Class

    Class PlayerState
        Public ReadOnly Position As Long = 0
        Public ReadOnly Score As Long = 0

        Public Sub New(position As Long, score As Long)
            Me.Position = position
            Me.Score = score
        End Sub

        Function Updated(a As Long) As PlayerState
            Dim pos = Position + a
            While pos > 10
                pos -= 10
            End While
            Return New PlayerState(pos, Score + pos)
        End Function

        Public Overrides Function ToString() As String
            Return $"({Position}, {Score})"
        End Function

        Public Overrides Function Equals(obj As Object) As Boolean
            If Not TypeOf obj Is PlayerState Then Return False
            Dim ps = CType(obj, PlayerState)
            Return Position.Equals(ps.Position) And Score.Equals(ps.Score)
        End Function

        Public Overrides Function GetHashCode() As Integer
            Return Position.GetHashCode() Xor Score.GetHashCode()
        End Function
    End Class


    Private Class DeterministicDie
        Dim _die = 0
        Dim _rolls = 0

        Public ReadOnly Property Rolls As Long
            Get
                Return _rolls
            End Get
        End Property

        Private Function Roll() As Long
            _die += 1
            _rolls += 1
            If _die > 100
                _die = 1
            End If
            Return _die
        End Function

        Function Roll3() As Long
            return Roll() + Roll() + Roll()
        End Function
    End Class

    Private Function Task1(p1 As Long, p2 As Long) As Long
        Dim die = New DeterministicDie()

        Dim game = New GameState(p1, p2)

        While True
            game = game.Updated(die.Roll3())
            If game.P1.Score >= 1000 Then Exit While
            game = game.Updated(die.Roll3())
            If game.P2.Score >= 1000 Then Exit While
        End While

        Return Math.Min(game.P1.Score, game.P2.Score)*die.Rolls
    End Function

    Private Function Expand(ByRef states As SortedDictionary(Of GameState, Long)) As Boolean
        Dim unexpandedStates = states.
                Where(Function(pair) Not pair.Key.P1.Score >= 21 And
                                     Not pair.Key.P2.Score >= 21)

        If not unexpandedStates.Any() Then Return False

        Dim nextExpansion = unexpandedStates.First()

        Dim count = nextExpansion.Value
        Dim state = nextExpansion.Key

        For i = 1 To 3
            For j = 1 To 3
                For k = 1 To 3
                    Dim newState = state.Updated(i + j + k)

                    If Not states.ContainsKey(newState) Then
                        states.Add(newState, 0)
                    End If
                    states.Item(newState) += count
                Next
            Next
        Next

        states.Remove(nextExpansion.Key)

        Return True
    End Function

    Private Class GameStateComparer
        Implements IComparer(of GameState)

        Public Function Compare(x As GameState, y As GameState) As Integer _
            Implements IComparer(Of GameState).Compare
            Dim a
            a = x.CurrentPlayer.Score.CompareTo(y.CurrentPlayer.Score)
            If a <> 0 Then Return a
            a = x.PreviousPlayer.Score.CompareTo(y.PreviousPlayer.Score)
            If a <> 0 Then Return a
            a = x.CurrentPlayer.Position.CompareTo(y.CurrentPlayer.Position)
            If a <> 0 Then Return a
            a = x.PreviousPlayer.Position.CompareTo(y.PreviousPlayer.Position)
            If a <> 0 Then Return a
            Return x.Player1Turn.CompareTo(y.Player1Turn)
        End Function
    End Class

    Private Function Task2(p1 As Long, p2 As Long) As Long
        Dim states As New SortedDictionary(Of GameState, Long)(New GameStateComparer())

        states.Add(New GameState(p1, p2), 1)

        While Expand(states)
        End While

        Dim player1Wins = states.
                Where(Function(pair) pair.Key.P1.Score > pair.Key.P2.Score).
                Select(Function(pair) pair.Value).
                Aggregate(Function (a, b) a + b)

        Dim player2Wins = states.
                Where(Function(pair) pair.Key.P2.Score > pair.Key.P1.Score).
                Select(Function(pair) pair.Value).
                Aggregate(Function (a, b) a + b)

        ' Console.WriteLine(player1Wins)
        ' Console.WriteLine(player2Wins)

        Return Math.Max(player1Wins, player2Wins)
    End Function
End Module
